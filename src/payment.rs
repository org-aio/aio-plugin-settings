use super::http;
use aio_plugin_identity_model::{PaymentChannelView, UpdatePaymentChannelRequest};
use az_ui_components::{
    admin::{AsyncResult, EditorDialog, StatusMessage},
    button::{Button, ButtonVariant},
    input::Input,
};
use dioxus::prelude::*;
use dioxus_icons::lucide::{CreditCard, RefreshCw};

/// 支付渠道设置：当前仅支付宝电脑网站支付，密钥只显示是否已保存。
#[component]
pub(super) fn PaymentSettings() -> Element {
    let mut revision = use_signal(|| 0_u64);
    let channels = use_resource(move || {
        let _ = revision();
        http::load_payment_channels()
    });
    let mut editing = use_signal(|| false);
    let mut status = use_signal(|| None::<Result<String, String>>);
    let result = channels.read().as_ref().cloned();
    let channel = result
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|channels| channels.first().cloned());
    rsx! {
        section { class: "workbench-settings__section", aria_label: "支付",
            div { class: "workbench-settings__section-header",
                div { h2 { CreditCard {} "支付" } p { "配置充值使用的支付渠道，供个人资料页钱包充值调用。" } }
                Button { variant: ButtonVariant::Outline, disabled: channel.is_none(),
                    onclick: move |_| editing.set(true), "配置支付宝" }
            }
            if let Some(message) = status() {
                StatusMessage { error: message.is_err(), message: message.unwrap_or_else(|message| message) }
            }
            match result {
                Some(Ok(channels)) if channels.is_empty() => rsx! { p { "暂无可用支付渠道。" } },
                Some(Ok(_)) => rsx! { PaymentSummary { channel: channel.clone().unwrap() } },
                Some(Err(error)) => rsx! {
                    p { role: "alert", "{error}" }
                    Button { variant: ButtonVariant::Outline, onclick: move |_| revision += 1, RefreshCw {} "重试" }
                },
                None => rsx! { p { class: "admin-meta", "正在加载支付渠道" } },
            }
        }
        if editing() {
            if let Some(channel) = channel {
                AlipayEditor {
                    channel,
                    on_close: move |_| editing.set(false),
                    on_saved: move |_| {
                        editing.set(false);
                        status.set(Some(Ok("支付宝配置已保存".into())));
                        revision += 1;
                    },
                }
            }
        }
    }
}

#[component]
fn PaymentSummary(channel: PaymentChannelView) -> Element {
    rsx! {
        dl { class: "admin-details",
            dt { "渠道" } dd { "支付宝" }
            dt { "状态" } dd {
                span { class: "admin-status", "data-enabled": channel.enabled.to_string(),
                    if channel.enabled { "已启用" } else { "未启用" }
                }
            }
            dt { "应用 ID" } dd { if channel.app_id.is_empty() { "未配置" } else { "{channel.app_id}" } }
            dt { "网关" } dd { class: "admin-code", "{channel.gateway}" }
            dt { "异步通知" } dd { if channel.notify_url.is_empty() { "未配置" } else { "{channel.notify_url}" } }
            dt { "同步跳转" } dd { if channel.return_url.is_empty() { "未配置" } else { "{channel.return_url}" } }
            dt { "应用私钥" } dd { if channel.has_private_key { "已保存" } else { "未配置" } }
            dt { "支付宝公钥" } dd { if channel.public_key.is_empty() { "未配置" } else { "已保存" } }
        }
    }
}

#[component]
fn AlipayEditor(
    channel: PaymentChannelView,
    on_close: Callback<()>,
    on_saved: Callback<()>,
) -> Element {
    let mut enabled = use_signal(|| channel.enabled);
    let mut app_id = use_signal(|| channel.app_id.clone());
    let mut gateway = use_signal(|| channel.gateway.clone());
    let mut notify_url = use_signal(|| channel.notify_url.clone());
    let mut return_url = use_signal(|| channel.return_url.clone());
    let mut seller_id = use_signal(|| channel.seller_id.clone());
    let mut public_key = use_signal(|| channel.public_key.clone());
    let mut private_key = use_signal(String::new);
    let mut clear_private_key = use_signal(|| false);
    let has_private_key = channel.has_private_key;
    rsx! {
        EditorDialog {
            title: "支付宝配置",
            description: "密钥仅保存到当前工作区，私钥加密后入库。留空保留已保存的私钥。",
            on_close,
            on_saved,
            save: move |_| -> AsyncResult<()> {
                let request = UpdatePaymentChannelRequest {
                    enabled: enabled(),
                    app_id: app_id(),
                    gateway: gateway(),
                    notify_url: notify_url(),
                    return_url: return_url(),
                    seller_id: seller_id(),
                    public_key: public_key(),
                    private_key: if clear_private_key() {
                        Some(String::new())
                    } else if private_key().is_empty() {
                        None
                    } else {
                        Some(private_key())
                    },
                };
                Box::pin(async move {
                    http::update_payment_channel("alipay", request).await.map(|_| ())
                })
            },
            label { class: "admin-field", span { "启用支付宝" }
                Input { r#type: "checkbox", aria_label: "启用支付宝", checked: enabled(),
                    onchange: move |event: FormEvent| enabled.set(event.checked()) }
            }
            label { class: "admin-field", span { "应用 ID（app_id）" }
                Input { aria_label: "支付宝应用 ID", value: app_id(), required: true,
                    oninput: move |event: FormEvent| app_id.set(event.value()) }
            }
            label { class: "admin-field", span { "网关地址" }
                Input { aria_label: "支付宝网关", value: gateway(),
                    oninput: move |event: FormEvent| gateway.set(event.value()) }
                small { "留空使用 https://openapi.alipay.com/gateway.do" }
            }
            label { class: "admin-field", span { "异步通知地址" }
                Input { aria_label: "支付宝异步通知地址", value: notify_url(),
                    oninput: move |event: FormEvent| notify_url.set(event.value()) }
                small { "应指向本站 /api/billing/alipay/notify" }
            }
            label { class: "admin-field", span { "同步跳转地址" }
                Input { aria_label: "支付宝同步跳转地址", value: return_url(),
                    oninput: move |event: FormEvent| return_url.set(event.value()) }
            }
            label { class: "admin-field", span { "商户 UID（可选）" }
                Input { aria_label: "支付宝商户 UID", value: seller_id(),
                    oninput: move |event: FormEvent| seller_id.set(event.value()) }
            }
            label { class: "admin-field", span { "支付宝公钥" }
                textarea { class: "dx-textarea", aria_label: "支付宝公钥", rows: "4", value: public_key(),
                    oninput: move |event: FormEvent| public_key.set(event.value()) }
                small { "用于校验异步通知签名，可粘贴裸 Base64 或 PEM。" }
            }
            label { class: "admin-field", span { "应用私钥" }
                textarea { class: "dx-textarea", aria_label: "支付宝应用私钥", rows: "4",
                    placeholder: if has_private_key { "已保存，留空保留" } else { "粘贴应用私钥" },
                    value: private_key(),
                    oninput: move |event: FormEvent| { private_key.set(event.value()); clear_private_key.set(false); } }
                if has_private_key {
                    label { class: "admin-field", span { "清除已保存的私钥" }
                        Input { r#type: "checkbox", aria_label: "清除支付宝私钥", checked: clear_private_key(),
                            onchange: move |event: FormEvent| clear_private_key.set(event.checked()) }
                    }
                }
            }
        }
    }
}

use super::http;
use az_ui_components::{
    admin::{PageHeader, PageSurface, RequestState},
    appearance::AppearanceSettings,
    badge::{Badge, BadgeVariant},
};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub(super) fn SettingsPage() -> Element {
    let mut resource = use_resource(http::load);
    let session = match resource.read().as_ref().cloned() {
        Some(Ok(value)) => value,
        Some(Err(error)) => {
            return rsx! { RequestState { error, on_retry: move |_| resource.restart() } };
        }
        None => return rsx! { RequestState {} },
    };
    rsx! {
        div { class: "workbench-settings",
            PageSurface {
                PageHeader { title: "设置", detail: "让工作台更适合你的习惯" }
                section { class: "workbench-settings__section", aria_label: "账户与工作区",
                    div { h2 { "账户与工作区" } p { "当前登录身份和正在使用的工作区。" } }
                    dl { class: "admin-details",
                        dt { "账户" } dd { "{session.display_name} (@{session.account})" }
                        dt { "工作区" } dd { "{session.tenant_label}" }
                    }
                    p { class: "admin-meta", "在账户菜单中打开个人资料或切换工作区。" }
                }
                section { class: "workbench-settings__section", aria_label: "外观",
                    div { h2 { "外观" } p { "调整主题和信息密度，修改即时生效。" } }
                    AppearanceSettings {}
                }
                section { class: "workbench-settings__section", aria_label: "关于",
                    div { h2 { "关于" } p { "AIO · 你的插件工作台" } }
                    p { "官方插件市场会自动展示已完成构建和发布的插件，无需配置市场源。" }
                    a { href: "https://github.com/zjarlin/aio-platform/blob/main/docs/plugin/README.md", target: "_blank", rel: "noopener noreferrer", "中文插件开发指南 ↗" }
                    details { class: "workbench-technical",
                        summary { "技术详情" }
                        dl { class: "admin-details", dt { "工作区 ID" } dd { code { "{session.tenant_id}" } } dt { "用户 ID" } dd { code { "{session.user_id}" } } }
                        h3 { "当前权限" }
                        div { class: "admin-badges", for permission in session.permissions { Badge { variant: BadgeVariant::Outline, "{permission}" } } }
                    }
                }
            }
        }
    }
}

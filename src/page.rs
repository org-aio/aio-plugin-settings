use super::{http, native::NativeSettings};
use az_dioxus_admin_shell::ApplicationSettings;
use az_ui_components::{
    admin::{PageHeader, PageSurface, RequestState},
    button::{Button, ButtonVariant},
};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub(super) fn SettingsPage() -> Element {
    let mut resource = use_resource(http::load);
    let extensions = use_context::<ApplicationSettings>();
    let mut selected = extensions.selected;
    let session = match resource.read().as_ref().cloned() {
        Some(Ok(value)) => value,
        Some(Err(error)) => {
            return rsx! { RequestState { error, on_retry: move |_| resource.restart() } };
        }
        None => return rsx! { RequestState {} },
    };
    let groups = extensions.groups.read();
    let current = selected().unwrap_or_default();
    let plugin = groups.iter().find(|group| group.page_id == current);
    let native = match current.as_str() {
        "appearance" | "about" => current.as_str(),
        _ => "general",
    };
    let active = plugin.map(|group| group.page_id.as_str()).unwrap_or(native);
    rsx! {
        div { class: "workbench-settings",
            PageSurface {
                PageHeader { title: "设置", detail: "管理工作台和已安装插件的设置" }
                div { class: "workbench-settings__layout",
                    nav { class: "workbench-settings__navigation", aria_label: "设置分组",
                        for (id, title) in [("general", "常规"), ("appearance", "外观"), ("about", "关于")] {
                            Button {
                                key: "{id}", variant: ButtonVariant::Ghost,
                                aria_pressed: (active == id).to_string(),
                                onclick: move |_| selected.set(Some(id.to_owned())),
                                "{title}"
                            }
                        }
                        if !groups.is_empty() {
                            span { class: "workbench-settings__navigation-heading", "插件" }
                        }
                        for group in groups.iter() {
                            Button {
                                key: "{group.page_id}", variant: ButtonVariant::Ghost,
                                aria_label: group.title.clone(),
                                aria_pressed: (active == group.page_id).to_string(),
                                onclick: {
                                    let id = group.page_id.clone();
                                    move |_| selected.set(Some(id.clone()))
                                },
                                span { class: "workbench-settings__navigation-label",
                                    span { "{group.title}" }
                                    small { class: "workbench-settings__source", "来自插件（{group.title}）" }
                                }
                            }
                        }
                    }
                    div { class: "workbench-settings__content",
                        if let Some(group) = plugin {
                            section {
                                key: "{group.page_id}",
                                class: "workbench-settings__section workbench-settings__plugin",
                                aria_label: format!("{}设置", group.title),
                                header {
                                    h2 { "{group.title}" }
                                    small { class: "workbench-settings__source", "来自插件（{group.title}）" }
                                }
                                {extensions.render.call(group.page_id.clone())}
                            }
                        } else {
                            NativeSettings { section: native.to_owned(), session }
                        }
                    }
                }
            }
        }
    }
}

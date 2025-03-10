/*
 * Copyright (c) 2024-2025 Oleg Okhotnikov
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_action_icons::MdSettings;
use dioxus_free_icons::icons::md_editor_icons::MdNotes;
use dioxus_free_icons::icons::md_file_icons::MdFolder;
use dioxus_free_icons::icons::md_navigation_icons::{MdArrowDropDown, MdArrowDropUp};
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};
use futures_util::StreamExt;

use crate::model::{default_sort_direction, Folder, FolderKey, Word, WordKey};
use crate::storage_global::get_storage;
use crate::ui::{CURRENT_TAB_DATA, msg_data_protection_is_set, msg_folder_was_deleted, msg_select_folder_first, msg_word_was_deleted};
use crate::ui::export_data::ExportData;
use crate::ui::folders::Folders;
use crate::ui::import_data::ImportData;
use crate::ui::settings::Settings;
use crate::ui::show_copyright::ShowCopyright;
use crate::ui::words::Words;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NavigationState {
    Folders,
    Words,
    Settings,
    ExportData,
    ImportData,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DataProtection {
    Protected,
    Unprotected,
}

pub(crate) const NAVIGATION_MESSAGE_ERROR: &str = text_red_500;
pub(crate) const NAVIGATION_MESSAGE_NOTIFICATION: &str = text_green_500;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct NavigationMessage {
    pub(crate) message: &'static str,
    pub(crate) color: &'static str,
}

#[component]
pub fn Navigation() -> Element {
    let mut navigation_state = use_signal(|| NavigationState::Folders);
    let mut navigation_message = use_signal(|| NavigationMessage::default());
    let _navigation_message_send = use_coroutine(move |mut rx| async move {
        while let Some(message) = rx.next().await {
            navigation_message.set(message);
        }
    });

    let navigation = use_coroutine(move |mut rx| async move {
        while let Some(state) = rx.next().await {
            navigation_state.set(state);
            navigation_message.set(NavigationMessage::default());
        }
    });

    let folder_str = use_signal(|| String::new());
    let folder_note_str = use_signal(|| String::new());
    let folder_error_str = use_signal(|| String::new());

    let folders_page_length = use_synced_storage::<LocalStorage, Option<u32>>(
        "folders_page_length".to_string(), || None::<u32>);
    let folders_offset = use_signal(|| None::<u32>);
    let folders_direction = use_signal(|| default_sort_direction.to_string());
    let refresh_folders = use_signal(|| 0u8);

    let show_add_folder_form = use_synced_storage::<LocalStorage, u8>(
        "show_add_folder_form".to_string(), || 255u8);

    let selected_folder_str = use_synced_storage::<LocalStorage, String>(
        "selected_folder".to_string(), || String::new());
    let selected_folder_error_str = use_signal(|| String::new());
    let mut word_str = use_signal(|| String::new());
    let word_error_str = use_signal(|| String::new());
    let mut word_class_str = use_signal(|| String::new());
    let mut note_str = use_signal(|| String::new());

    let words_page_length = use_synced_storage::<LocalStorage, Option<u32>>(
        "page_length".to_string(), || None::<u32>);
    let words_page_offset = use_signal(|| None::<u32>);
    let words_direction = use_signal(|| default_sort_direction.to_string());
    let refresh_words = use_signal(|| 0u8);

    let show_add_word_form = use_synced_storage::<LocalStorage, u8>(
        "show_add_word_form".to_string(), || 255u8);

    let data_protection = use_signal(|| DataProtection::Protected);
    let data_protection_error = move ||
    navigation_message.set(NavigationMessage {
        message: msg_data_protection_is_set,
        color: NAVIGATION_MESSAGE_ERROR,
    });

    let _delete_word = use_coroutine(move |mut rx: UnboundedReceiver<WordKey>| {
        to_owned![refresh_words, data_protection_error];
        async move {
            while let Some(word_key) = rx.next().await {
                match data_protection() {
                    DataProtection::Protected => data_protection_error(),
                    DataProtection::Unprotected => {
                        let _ = get_storage().delete_by_id::<Word>(word_key.id).await;
                        refresh_words.toggle();
                        navigation_message.set(NavigationMessage {
                            message: msg_word_was_deleted,
                            color: NAVIGATION_MESSAGE_ERROR,
                        });
                    }
                }
            }
        }
    });

    let _delete_folder = use_coroutine(move |mut rx: UnboundedReceiver<FolderKey>| {
        to_owned![refresh_folders, data_protection_error];
        async move {
            while let Some(folder_key) = rx.next().await {
                match data_protection() {
                    DataProtection::Protected => data_protection_error(),
                    DataProtection::Unprotected => {
                        let _ = get_storage().delete_by_id::<Folder>(folder_key.id).await;
                        refresh_folders.toggle();
                        navigation_message.set(NavigationMessage {
                            message: msg_folder_was_deleted,
                            color: NAVIGATION_MESSAGE_ERROR,
                        });
                    }
                }
            }
        }
    });

    let current_tab_data = use_memo(move || CURRENT_TAB_DATA());

    use_effect(move || {
        word_str.set(current_tab_data().word);
        word_class_str.set(current_tab_data().word_class);
        note_str.set(current_tab_data().phonetics);
    });

    rsx! {
        div { class: class!(text_lg),
            tabindex: "-1",
            position: "relative",
            margin_top: "5px",
            form { action: "",
                tabindex: "-1",
                onsubmit: move |event| event.stop_propagation(),
                div { class: class!(flex flex_row items_baseline),
                    tabindex: "-1",
                    label { title: "Folders",
                        button { class: class!(btn btn_sm flex_none),
                            onclick: move |_| navigation.send(NavigationState::Folders),
                            Icon { icon: MdFolder }
                            "Folders"
                        }
                    }
                    ShowFormButton {
                        title: "Show add folder form",
                        navigation_state: navigation_state,
                        form_state: NavigationState::Folders,
                        show_form: show_add_folder_form,
                    }
                    label { title: "Words",
                        button { class: class!(btn btn_sm flex_none),
                            margin_left: "1px",
                            margin_right: "1px",
                            onclick: move |_| {
                                if selected_folder_str().len() != 0 {
                                    navigation.send(NavigationState::Words);
                                } else {
                                    navigation_message.set(NavigationMessage {
                                        message: msg_select_folder_first,
                                        color: NAVIGATION_MESSAGE_ERROR,
                                    });
                                }
                            },
                            Icon { icon: MdNotes }
                            "Words"
                        }
                    }
                    ShowFormButton {
                        title: "Show add word form",
                        navigation_state: navigation_state,
                        form_state: NavigationState::Words,
                        show_form: show_add_word_form,
                    }
                    label { title: "Settings",
                        button { class: class!(btn btn_sm flex_none),
                            margin_left: "5px",
                            onclick: move |_| navigation.send(NavigationState::Settings),
                            Icon { icon: MdSettings }
                        }
                    }
                }

                p { class: class!(text_xs pt_2 navigation_message().color),
                    {navigation_message().message}
                }
            }
            match navigation_state() {
                NavigationState::Folders => {
                    rsx! {
                        Folders {
                            folder_str: folder_str,
                            folder_note_str: folder_note_str,
                            folder_error_str: folder_error_str,
                            selected_folder_str: selected_folder_str,
                            page_length: folders_page_length,
                            offset: folders_offset,
                            direction: folders_direction,
                            refresh_folders: refresh_folders,
                            show_add_folder_form: show_add_folder_form,
                            words_page_offset: words_page_offset,
                        }
                    }
                }
                NavigationState::Words => {
                    rsx! {
                        Words {
                            selected_folder_str: selected_folder_str,
                            selected_folder_error_str: selected_folder_error_str,
                            word_str: word_str,
                            word_error_str: word_error_str,
                            word_class_str: word_class_str,
                            note_str: note_str,
                            page_length: words_page_length,
                            offset: words_page_offset,
                            direction: words_direction,
                            refresh_words: refresh_words,
                            show_add_word_form: show_add_word_form,
                        }
                    }
                }
                NavigationState::Settings => {
                    rsx! {
                        Settings {
                            folders_page_length: folders_page_length,
                            folders_offset: folders_offset,
                            words_page_length: words_page_length,
                            words_offset: words_page_offset,
                            data_protection: data_protection,
                        }
                    }
                }
                NavigationState::ExportData => {
                    rsx! {
                        ExportData {}
                    }
                }
                NavigationState::ImportData => {
                    rsx! {
                        ImportData {}
                    }
                }
            }
        }
        if navigation_state() == NavigationState::Settings {
            ShowCopyright {}
        }
    }
}

#[component]
pub fn ShowFormButton(
    title: &'static str,
    navigation_state: Signal<NavigationState>,
    form_state: NavigationState,
    show_form: Signal<u8>,
) -> Element {
    rsx! {
        if navigation_state() == form_state {
            label { title: title,
                button { class: class!(btn btn_sm flex_none pl_2 pr_2),
                    margin_left: "1px",
                    margin_right: "1px",
                    onclick: move |_| show_form.toggle(),
                    if show_form() == 0u8 {
                        Icon { icon: MdArrowDropDown }
                    } else {
                        Icon { icon: MdArrowDropUp }
                    }
                }
            }
        } else {
            span { class: class!(flex_none w_10 pl_2 pr_2) }
        }
    }
}

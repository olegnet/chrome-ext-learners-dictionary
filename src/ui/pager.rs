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

use crate::ui::{MAX_WORD_INDEX, SELECTED_WORD_INDEX};
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[derive(Clone, PartialEq)]
pub(crate) enum PagerMode {
    Folders,
    Words,
}

#[component]
pub(crate) fn Pager(
    mode: ReadOnlySignal<PagerMode>,
    page_length: Signal<Option<u32>>,
    offset: Signal<Option<u32>>,
    direction: Signal<String>,
    count: u32,
) -> Element {
    if mode() == PagerMode::Words {
        if let Some(v) = SELECTED_WORD_INDEX() {
            if v < 0 {
                page_left(mode, page_length, offset);
            } else if v >= MAX_WORD_INDEX() {
                page_right(mode, count, page_length, offset, true);
            }
        }
    }

    let pager = match page_length() {
        Some(length) => {
            let page_number = offset().unwrap_or(0) / length + 1;
            let total_pages = (count + length - 1) / length;
            let last_page_offset = (total_pages - 1) * length;
            rsx! {
                div { class: class!(flex_none self_center w_5),
                    label { title: "First page",
                        button {
                            onclick: move |_| first_page(mode, offset),
                            "\u{21E4}"
                        }
                    }
                }
                div { class: class!(flex_none self_center w_5),
                    label { title: "Page left",
                        button {
                            onclick: move |_| page_left(mode, page_length, offset),
                            "\u{2190}"
                        }
                    }
                }
                div { class: class!(flex_none self_center),
                    label { title: "Page number",
                        "{page_number}"
                    }
                }
                div { class: class!(flex_none self_center),
                    "/"
                }
                div { class: class!(flex_none self_center),
                    label { title: "Total pages",
                        "{total_pages}"
                    }
                }
                div { class: class!(flex_none self_center w_5),
                    label { title: "Page right",
                        button {
                            onclick: move |_| page_right(mode, count, page_length, offset, false),
                            "\u{2192}"
                        }
                    }
                }
                div { class: class!(flex_none self_center w_5),
                    label { title: "Last page",
                        button {
                            onclick: move |_| last_page(mode, offset, last_page_offset),
                            "\u{21E5}"
                        }
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    rsx! {
        div { class: class!(flex flex_row gap_2 items_baseline),
            div { class: class!(flex_none self_center w_5),
                SortElement {
                    mode,
                    title: "Sort order: ascending",
                    element: "\u{2191}",
                    direction: direction,
                }
            }
            div { class: class!(flex_none self_center w_5),
                SortElement {
                    mode,
                    title: "Sort order: descending",
                    element: "\u{2193}",
                    direction: direction,
                }
            }
            {pager}
            div { class: class!(flex_none self_center),
                label { title: "Count",
                    "{count}"
                }
            }
        }
    }
}

#[component]
fn SortElement(
    mode: ReadOnlySignal<PagerMode>,
    title: &'static str,
    element: &'static str,
    direction: Signal<String>,
) -> Element {
    let cls = if direction().as_str() == element { invert } else { "" };

    rsx! {
        label { title: "{title}",
            button { class: class!(cls),
                onclick: move |_| {
                    direction.set(element.to_string());
                    if mode() == PagerMode::Words {
                        *SELECTED_WORD_INDEX.write() = None;
                    }
                },
                "{element}"
            }
        }
    }
}

fn first_page(mode: ReadOnlySignal<PagerMode>, mut offset: Signal<Option<u32>>) {
    offset.set(None);
    set_index_to_zero(mode);
}

fn last_page(
    mode: ReadOnlySignal<PagerMode>,
    mut offset: Signal<Option<u32>>,
    last_page_offset: u32,
) {
    offset.set(Some(last_page_offset));
    set_index_to_zero(mode);
}

fn page_left(
    mode: ReadOnlySignal<PagerMode>,
    page_length: Signal<Option<u32>>,
    mut offset: Signal<Option<u32>>,
) {
    if page_length() == None {
        set_index_to_zero(mode);
        return;
    }
    let page_length = page_length().unwrap() as i32;
    let new_off = offset().unwrap_or(0) as i32 - page_length;
    if new_off >= 0 {
        offset.set(Some(new_off as u32));
        *MAX_WORD_INDEX.write() = page_length;
        set_index_to_max(mode);
    } else {
        set_index_to_zero(mode);
    }
}

fn page_right(
    mode: ReadOnlySignal<PagerMode>,
    count: u32,
    page_length: Signal<Option<u32>>,
    mut offset: Signal<Option<u32>>,
    is_key_pressed: bool,
) {
    if page_length() == None {
        set_index_to_one_step_back(mode);
        return;
    }
    let page_length = page_length().unwrap();
    let new_off = offset().unwrap_or(0) + page_length;
    if new_off < count {
        offset.set(Some(new_off));
        set_index_to_zero(mode);
    } else if is_key_pressed {
        set_index_to_one_step_back(mode);
    }
}

fn set_index_to_zero(mode: ReadOnlySignal<PagerMode>) {
    if mode() != PagerMode::Words {
        return;
    }
    if let Some(_) = SELECTED_WORD_INDEX() {
        *SELECTED_WORD_INDEX.write() = Some(0)
    }
}

fn set_index_to_max(mode: ReadOnlySignal<PagerMode>) {
    if mode() != PagerMode::Words {
        return;
    }
    if let Some(_) = SELECTED_WORD_INDEX() {
        *SELECTED_WORD_INDEX.write() = Some(MAX_WORD_INDEX() - 1);
    }
}

fn set_index_to_one_step_back(mode: ReadOnlySignal<PagerMode>) {
    if mode() != PagerMode::Words {
        return;
    }
    SELECTED_WORD_INDEX.with_mut(move |v| match *v {
        Some(x) => *v = Some(x - 1),
        None => {}
    });
}

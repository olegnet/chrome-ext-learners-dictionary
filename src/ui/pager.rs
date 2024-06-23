/*
 * Copyright (c) 2024 Oleg Okhotnikov
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

#[component]
pub(crate) fn Pager(
    page_length: Signal<Option<u32>>,
    offset: Signal<Option<u32>>,
    direction: Signal<String>,
    count: u32,
) -> Element {
    let pager = match page_length() {
        Some(length) => {
            let page_number = offset().unwrap_or(0) / length + 1;
            let total_pages = (count + length - 1) / length;
            let last_page_offset = (total_pages - 1) * length;
            rsx! {
                div { class: class!(flex_none self_center w_5),
                    label { title: "First page",
                        a { href: "#",
                            onclick: move |_| offset.set(None),
                            "\u{21E4}"
                        }
                    }
                }
                div { class: class!(flex_none self_center w_5),
                    label { title: "Page left",
                        a { href: "#",
                            onclick: move |_| page_left(page_length, offset),
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
                        a { href: "#",
                            onclick: move |_| page_right(count, page_length, offset),
                            "\u{2192}"
                        }
                    }
                }
                div { class: class!(flex_none self_center w_5),
                    label { title: "Last page",
                        a { href: "#",
                            onclick: move |_| offset.set(Some(last_page_offset)),
                            "\u{21E5}"
                        }
                    }
                }
                // div { class: class!(flex_none self_center),
                //     label { title: "Offset",
                //         "{offset().unwrap_or(1)}"
                //     }
                // }
                // div { class: class!(flex_none self_center),
                //     "/"
                // }
            }
        },
        None => None,
    };

    rsx! {
        div { class: class!(flex flex_row gap_2 items_baseline),
            div { class: class!(flex_none self_center w_5),
                SortElement {
                    title: "Sort order: ascending",
                    element: "\u{2191}",
                    direction: direction,
                }
            }
            div { class: class!(flex_none self_center w_5),
                SortElement {
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
    title: &'static str,
    element: &'static str,
    direction: Signal<String>,
) -> Element {
    let cls = if direction().as_str() == element { invert } else { "" };

    rsx! {
        label { title: "{title}",
            a { class: class!(cls),
                href: "#",
                onclick: move |_| direction.set(element.to_string()),
                "{element}"
            }
        }
    }
}

fn page_left(page_length: Signal<Option<u32>>, mut offset: Signal<Option<u32>>) {
    if page_length() == None {
        return;
    }
    let new_off = (offset().unwrap_or(0) as i32) - (page_length().unwrap() as i32);
    if new_off >= 0 {
        offset.set(Some(new_off as u32));
    }
}

fn page_right(count: u32, page_length: Signal<Option<u32>>, mut offset: Signal<Option<u32>>) {
    if page_length() == None {
        return;
    }
    let new_off = offset().unwrap_or(0) + page_length().unwrap();
    if new_off < count {
        offset.set(Some(new_off));
    }
}

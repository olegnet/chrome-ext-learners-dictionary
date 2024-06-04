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

use crate::model::sort_directions;

#[component]
pub(crate) fn Pager(
    page_length: Signal<Option<u32>>,
    offset: Signal<Option<u32>>,
    direction: Signal<String>,
    count: u32,
) -> Element {
    let sort_directions_rendered = sort_directions.iter().map(|(dir, _)| {
        rsx! {
            option { value: "{dir}",
                "{dir}"
            }
        }
    });

    rsx! {
        div {
            form { action: "",
                onsubmit: move |event| event.stop_propagation(),
                select { name: "order",
                    id: "order",
                    onchange: move |event| direction.set(event.value()),
                    {sort_directions_rendered}
                }
                if page_length() != None {
                    // FIXME A <label> isn't associated with a form field.
                    label { class: class!(text_green_500),
                        margin_left: "5px",
                        title: "<<",
                        a { href: "#",
                            onclick: move |_| page_left(page_length, offset),
                            "<<"
                        }
                    }
                    label { class: class!(text_green_500),
                        margin_left: "5px",
                        title: ">>",
                        a { href: "#",
                            onclick: move |_| page_right(count, page_length, offset),
                            ">>"
                        }
                    }
                }
                label { class: class!(text_red_500),
                    margin_left: "10px",
                    title: "count",
                    "{count}"
                }
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

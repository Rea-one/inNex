use leptos::prelude::*;
use leptos::html::Div;
use leptos::web_sys::{KeyboardEvent, WheelEvent};

use crate::model::words::mexp;
use crate::model::visual::meditor;


#[component]
pub fn Meditor(me: RwSignal<mexp::Mexp>, core: RwSignal<meditor::MeditorModel>) 
-> impl IntoView {
    let (content, set_content) = signal(Vec::<String>::new());
    let div_ref = NodeRef::<Div>::new();
    
    // 处理键盘事件
    let handle_key_down = move |e: KeyboardEvent| {
        match e.key().as_str() {
            "ArrowUp" => {
                let current_offset = core.read().get_offset();
                core.write().set_offset(current_offset - 1);
            }
            "ArrowDown" => {
                let current_offset = core.read().get_offset();
                core.write().set_offset(current_offset + 1);
            }
            _ => {}
        }
    };
    
    // 处理滚轮事件
    let handle_wheel = move |e: WheelEvent| {
        let delta_y = e.delta_y();
        if delta_y > 0.0 {
            let current_offset = core.read().get_offset();
            core.write().set_offset(current_offset + 1);
        } else if delta_y < 0.0 {
            let current_offset = core.read().get_offset();
            core.write().set_offset(current_offset - 1);
        }
    };
    
    // 响应式更新内容
    Effect::new(move |_| {
        let at = core.read().at;
        let offset = core.read().get_offset();
        let new_content = me.write().aquest(at, offset);
        set_content.set(new_content);
    });

    view! {
        <div class="meditor" tabindex="0" on:keydown=handle_key_down on:wheel=handle_wheel node_ref=div_ref>
            <For
                each=move || content.get()
                key=|token| token.clone()
                children=move |token| {
                    view! {
                        <span>{token}</span>
                    }
                }
            />
        </div>
    }
}
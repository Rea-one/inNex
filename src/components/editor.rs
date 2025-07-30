use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use leptos::logging;

/// 一个简单的文本编辑器组件
#[component]
pub fn SimEditor(
    /// 编辑器文本
    texts: String,
    /// 当文本改变时调用的回调函数
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    let (text, set_text) = signal(texts);

    let handle_input = move |ev| {
        let value = event_target_value(&ev);
        set_text.set(value.clone());
    };

    let handle_keydown = move |ev: KeyboardEvent| {
        // 这里可以处理特定的键盘事件
        // 例如：Ctrl+S 保存等
        if ev.ctrl_key() && ev.key() == "s" {
            // 阻止浏览器默认的保存行为
            ev.prevent_default();
            // 可以在这里添加保存逻辑
            logging::log!("用户按下了 Ctrl+S");
        }
    };

    view! {
        <div class="editor-container">
            <textarea
                class="editor-textarea"
                on:input=handle_input
                on:keydown=handle_keydown
                prop:value=move || text.get()
                rows="127"
                cols="127"
                placeholder="文本待输入"
            />
        </div>
    }
}
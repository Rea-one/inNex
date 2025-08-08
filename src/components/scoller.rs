use leptos::ev::MouseEvent;
use leptos::web_sys::WheelEvent;
use leptos::{component, IntoView};
use leptos::prelude::*;

pub struct ScollerModel {
    total_lines: usize,
    at: usize,
}

impl ScollerModel {
    pub fn new(total_lines: usize) -> Self {
        ScollerModel { total_lines, at: 0 }
    }

    pub fn set_total_lines(&mut self, total_lines: usize) {
        self.total_lines = total_lines;
        self.at = self.at.min(total_lines.saturating_sub(1));
    }

    pub fn scroll_to(&mut self, line: usize) {
        self.at = line.min(self.total_lines.saturating_sub(1));
    }

    pub fn scroll_by(&mut self, offset: i32) {
        let new_line = if offset < 0 {
            self.at.saturating_sub((-offset) as usize)
        } else {
            self.at.saturating_add(offset as usize)
        };
        self.at = new_line.min(self.total_lines.saturating_sub(1));
    }

    pub fn at(&self) -> usize {
        self.at
    }

    pub fn total_lines(&self) -> usize {
        self.total_lines
    }
}

#[derive(Clone)]
pub struct Scoller {
    width: usize,
    height: usize,
    me: RwSignal<MeScoller>,
    thumb: f32,
}

struct MeScoller {
    total_lines: usize,
    abs_pos: usize,
    rel_pos: usize,
    at: usize,
}

impl MeScoller {
    fn new() -> Self {
        MeScoller {
            total_lines: 0,
            abs_pos: 0,
            rel_pos: 0,
            at: 0,
        }
    }

    pub fn drag(&mut self, offset: i32) {
        let tmp_at = self.at as i32 + offset;
        if tmp_at < 0 {
            self.at = 0;
        } else if tmp_at >= self.total_lines as i32 {
            self.at = self.total_lines - 1;
        } else {
            self.at = tmp_at as usize;
        }
    }
    pub fn rel_drag(&mut self, offset: f32) {
        let tmp_at = self.at as i32 + (offset * self.total_lines as f32) as i32;
        if tmp_at < 0 {
            self.at = 0;
        } else if tmp_at >= self.total_lines as i32 {
            self.at = self.total_lines - 1;
        } else {
            self.at = tmp_at as usize;
        }
    }

    pub fn abs_to(&mut self, at: usize) {
        if at >= self.total_lines {
            self.at = self.total_lines - 1;
        } else {
            self.at = at;
        }
    }
    pub fn rel_to(&mut self, at: f32) {
        if at < 0.0 {
            self.at = 0;
        } else {
            let tmp_at = (at * self.total_lines as f32) as usize;
            self.abs_to(tmp_at);
        }
    }

    /// 获取当前滚动位置
    pub fn get_at(&self) -> usize {
        self.at
    }
}



impl Scoller {
    pub fn new(width: usize, height: usize) -> Self {
        Scoller {
            width,
            height,
            me: RwSignal::new(MeScoller::new()),
            thumb: 0.0,
        }
    }

    pub fn drag(&self, offset: i32) {
        self.me.write().drag(offset);
    }

    pub fn rel_drag(&self, offset: f32) {
        self.me.write().rel_drag(offset);
    }

    pub fn abs_to(&self, at: usize) {
        self.me.write().abs_to(at);
    }

    pub fn rel_to(&self, at: f32) {
        self.me.write().rel_to(at);
    }

    pub fn get_at(&self) -> usize {
        self.me.read().get_at()
    }
}

#[component]
pub fn scoller(core: Scoller) -> impl IntoView {
    let core_drag = core.clone();
    let core_scroll = core;

    let on_drag = move |e: MouseEvent| {
        core_drag.drag(e.movement_y());
    };

    let on_scroll = move |e: WheelEvent| {
        core_scroll.rel_drag(e.delta_y() as f32 / 100.0);
    };

    view! {
        <div class="scroller">
            
        </div>
    }
}

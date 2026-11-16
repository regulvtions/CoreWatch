//! A short fade-in for the newly selected page. Navigation remains fully opaque.
use eframe::egui;
use crate::Tab;
const DURATION: f64 = 0.16;

#[derive(Clone)]
struct Fade { tab: Tab, started: Option<f64> }
impl Fade {
    fn sample(&mut self, tab: Tab, now: f64) -> f32 {
        if tab != self.tab { self.tab = tab; self.started = Some(now); }
        let Some(started) = self.started else { return 1.0; };
        let t = ((now-started)/DURATION).clamp(0.0,1.0) as f32;
        if t >= 1.0 { self.started = None; return 1.0; }
        // Smoothstep avoids an abrupt start or stop.
        t*t*(3.0-2.0*t)
    }
}

pub fn opacity(ctx: &egui::Context, tab: Tab) -> f32 {
    let now=ctx.input(|i|i.time);
    let alpha=ctx.data_mut(|data| {
        let fade=data.get_temp_mut_or_insert_with(egui::Id::new("corewatch-tab-fade"),||Fade{tab,started:None});
        fade.sample(tab,now)
    });
    if alpha < 1.0 { ctx.request_repaint_after(std::time::Duration::from_millis(16)); }
    alpha
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn first_page_is_visible_and_same_tab_does_not_restart_fade() {
        let mut fade=Fade{tab:Tab::Monitor,started:None};
        assert_eq!(fade.sample(Tab::Monitor,0.0),1.0);
        assert_eq!(fade.sample(Tab::Rgb,1.0),0.0);
        let middle=fade.sample(Tab::Rgb,1.08);assert!((middle-0.5).abs()<0.001);
        assert_eq!(fade.sample(Tab::Rgb,1.2),1.0);assert!(fade.started.is_none());
        assert_eq!(fade.sample(Tab::Rgb,2.0),1.0);
    }
    #[test] fn rapid_switch_uses_latest_tab_and_finishes_after_background_pause() {
        let mut fade=Fade{tab:Tab::Monitor,started:None};
        fade.sample(Tab::Fans,1.0);
        assert_eq!(fade.sample(Tab::Options,1.04),0.0);
        assert!(fade.sample(Tab::Options,1.1)>0.0);
        assert_eq!(fade.sample(Tab::Options,10.0),1.0);
    }
}

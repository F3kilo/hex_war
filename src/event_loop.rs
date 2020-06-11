use crate::app::App;
use winit::event_loop::EventLoop;

pub fn run_event_loop<A>(event_loop: EventLoop<A::UserEvent>, mut app: A) -> !
where
    A: App + 'static,
{
    event_loop.run(move |event, event_loop_wt, control_flow| {
        *control_flow = app.process_event(event, event_loop_wt);
    });
}

use crate::app::{status::Status, App, WinitEventAdaptor};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run_event_loop<A, E>(event_loop: EventLoop<()>, mut app: A, event_adaptor: E) -> !
where
    A: App + 'static,
    E: WinitEventAdaptor<AppEvent = A::Event> + 'static,
{
    event_loop.run(move |event, event_loop_wt, control_flow| {
        *control_flow = ControlFlow::Poll;

        let adapted_event = event_adaptor.adapt_event(event);

        if let Ok(app_event) = adapted_event {
            if let Status::Finish = app.process_event(app_event, event_loop_wt) {
                *control_flow = ControlFlow::Exit;
            }
            return;
        }

        let event = adapted_event.err().unwrap();

        match event {
            Event::MainEventsCleared => {
                if let Status::Finish = app.update(event_loop_wt) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::RedrawRequested(window_id) => app.draw(window_id),
            _ => {}
        };
    });
}

use ggez::{timer, Context};

pub fn print_fps(msg: &str, ctx: &mut Context) {
    println!(
        "[{}] ticks: {}\tfps: {}\tdelta: {:?}",
        msg,
        timer::ticks(ctx),
        timer::fps(ctx),
        timer::delta(ctx)
    );
}

use polaroid::frame::{new_frame, Drawable};
use polaroid::invaders::Invaders;
use polaroid::player::Player;
use polaroid::{frame, render};
use rusty_audio::Audio;
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};

use std::sync::mpsc;
use std::thread::current;
use std::{
    error::Error,
    time::{Duration, Instant},
    {io, thread}
};

fn main() -> Result <(), Box<dyn Error>> // Allows us to return concrete error types in a reference box as printables, but removes any potentially valuable type information. 
{
    let mut audio = Audio::new();
    audio.add("explode", "audio/explode.wav");
    audio.add("lose", "audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");  
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; // ? unpacks the result if it returns a Result of Ok, and returns the error if not.
    stdout.execute(Hide)?;

    // Render loop in a seperate thread.
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || 
    {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let current_frame = match render_rx.recv()
            {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });
    
    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    
    'gameloop: loop 
    {
        // Per frame init.
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame();

        // Input
        while crossterm::event::poll(std::time::Duration::default())?
        {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()?
            {
                match key_event.code 
                {
                    crossterm::event::KeyCode::Left => player.move_left(),
                    crossterm::event::KeyCode::Right => player.move_right(),
                    crossterm::event::KeyCode::Char(' ') | crossterm::event::KeyCode::Enter =>
                    {
                        if player.shoot()
                        {
                            audio.play("pew");
                        }
                    }

                    crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') =>
                    {
                        audio.play("lose");
                        break 'gameloop;
                    }

                    _ => { }
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta)
        {
            audio.play("move");
        }

        if player.detect_hits(&mut invaders)
        {
            audio.play("explode");
        }

        // Draw and render section.
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables
        {
            drawable.draw(&mut current_frame);
        }

        let _ = render_tx.send(current_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose.
        if invaders.all_killed()
        {
            audio.play("win");
            break 'gameloop;
        }

        if invaders.reached_bottom()
        {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(()) // Result type.
}

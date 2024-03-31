Rusify

# Order of events
- Read Rust book
- While reading rust book participated in advent of code
- Finished reading rust book
- Started researching frameworks for senior project
- Decide on making audio app with Rodio and Eframe
- Initial testing of Rodio functionality
- Start implementing basic functionality with ui
- Realizing I need threads for the project
- Experiment with thread designs in order to send messages but fail.
- Implement the audio thread core using the MPSC to give messages to a command handling enum, along with a singleton for the thread
- Refactor out all audio logic into the new audio thread and rewrite a lot of the program
- Once refactored implementation of new features becomes much easier.
- Radically enhance the ui adding new features along the way
- First features to implement was the bottom bar, including play/pause, and a volume slider
- Removed the intended support of MP3 files as they don’t contain the Metadata necessary to support the apps functionality
- Begin experimenting with the track progress slider but decide to shelf it until other progress has been made due to difficulty.
- Add playlist and queue support and add the ui to support
- Start working on the track progress bar again
- Make breakthroughs on the progress bar until it is fully complete.


# Questions

What did you learn?

What would you do differently?

What technologies did you use, and why?

How is your software organized, and now that you're done was that the right choice? Include diagrams if appropriate.

What was the hardest part of the project?

Where there any complex data structures or algorithms. Include diagrams if appropriate.

Was the project about as hard as you predicted? If not, where was the error?

# Paper
The inception of "Rusify" stemmed from the desire to explore the Rust programming language and its application in building a real-world desktop application. The Rust programming language has garnered praise over the last few years for its focus on safety, speed, and concurrency. Developers have also been impressed with it as it has received the title of “most loved programming language” every year from 2016 to 2023 in the Stack Overflow Developer Survey. This project's objective was to explore the Rust language by creating a music player with seamless audio playback and a user-friendly interface, leveraging Rust's strengths in memory safety, concurrency, and performance.

I began to learn Rust through the freely available Rust Programming Book published by the creators of the language. I built my foundation of knowledge of Rust by reading this book over the course of a few weeks. It provided insights into Rust's memory management, ownership model, and concurrency mechanisms. Additionally, I participated in Advent of Code challenges during the same time to apply what I was learning in the book to actual coding problems.

The book brought me through all of the basic features of the language, and touched on some very important points. Firstly is how Rust handles memory, which is vastly different from other languages. There is no garbage collector, but it doesn’t require you to manually allocate and deallocate memory like C. Instead Rust uses the idea of ownership, and a set of rules that must be followed, more on this later. Other helpful concepts were pattern matching, how Rust handles concurrency, and some of the abstraction the language implements to remove bugs from code like the iterator pattern among other things.

Once I felt confident in my basic Rust abilities, I decided to start looking for a graphical framework that would support the use case of my application. I stumbled upon egui, for which eframe is a framework and decided that fit the application needs the best. Egui/Eframe are a cross platform and web/desktop immediate GUI. Eframe is not a framework designed for game development but there are frameworks/engines that do utilize egui’s speed like bevy.

I also began looking into audio frameworks and found that Rodio was exactly what I was looking for as an audio parsing library for rust. It handled all that I needed for the project. I started out the project by first testing the audio framework. Once I got it working outside of the graphical application I decided to transplant the logic into the template app and attach some functionality to ui. I gave that a test and the music played, however the entire ui of the app locked up while music was playing. I realized then that the music was playing on the same thread as the ui and if I wanted the app to behave as intended I would need to run on multiple threads.

I naively began writing the code to handle all of the audio logic mixed in with all of the UI code since that was where the events were being triggered, that meant the audio thread was completely coupled with the ui, immediately bringing development to a crawl. I was having many difficulties trying to sync everything up, and even to have only one audio thread when I realized I had to abstract the audio thread to progress.

Up until this point I had managed to implement a few underwhelming features like playing a single song, and play/pause, but they were a mess and still very buggy. Nothing was getting fixed from that point so I decided it was time for the first refactor, where all effort for a short time was directed to improving the workability of the codebase and clearing technical debt left by the prototyping of the initial app.

I created the audio_thread.rs file which quickly became the new heart of the app. The first problem that needed to be solved was making only one instance of the audio thread possible. This was handled by implementing the singleton pattern to ensure the instantiation of only one new thread. Next was solving the problem of communication between threads. Originally the only way I was using the audio_thread was by creating it and having it run code upon its initialization, which was not a dynamic or capable system at all. Remembering the section on threading from the Rust programming book, Rust provides an extremely eloquent way to solve the issue of passing messages between threads: The Multi Producer Single Consumer (MPSC). This method uses channels in order to send and receive commands as I heavily utilized the command pattern to run the audio thread. I set up an enum to handle all of the commands and receive the messages from the MPSC queue. This handles all concurrency problems extremely effectively and unlike other times when dealing with multiple threads this was incredibly concise and idiomatic.

Now that a solid system of adding new features was implemented, I extracted all remaining logic from the ui and was able to add them feature by feature into the new enum.

Quote from the book which was taken from the go language documentation
“Do not communicate by sharing memory; instead, share memory by communicating.”

The most difficult feature to implement in the app was the interactive progress bar. There were many times where I thought it was bug free only for it to break in the strangest of ways. The difficulty in implementation had a lot to do with the fact that Rodio did not provide any interface to how long the song has been playing, the only thing that saved the feature from being chopped entirely was that I was able to grab the duration of the track, which would allow for me to compute everything necessary in order to make the progress bar work properly.

The first changes to the audio thread was adding the GetDuration audio command so I could get the track duration from the audio thread in order to display it in the application. Next was to have the slider move while the track was playing, so I had to implement another getter, GetProgress, which would give the accurate amount of time that the track had been playing, so that the slider moved as time passed. This was quite simple for the case of music being played without any stops, as soon as the music started playing the INSTANT was recorded, and the elapsed time since that instant was the track progress. This however wouldn’t remain so simple as pausing messes with the time as well.





```rust
pub enum AudioCommand {
    GetProgress(Sender<Duration>),
    GetState(Sender<AudioState>),
    GetTrackDuration(Sender<Duration>),
    Pause,
    Play,
    PlaySong(String),
    SetProgress(f32, String),
    SetVolume(f32),
    Skip,
}
```
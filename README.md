# Rustify

---

## Notes

- Need to have some determination when a track is over so that the queue can
properly track and the play pause determination is correct.




Todo

Probably want to start extracting out all of the ui elements instead of them
all being in the app.rs

Also need to figure out where I want to put the track progress timer/logic


Final Additions that need to be made are the playist support, and possibly making
an attempt at a track progress bar


Some new goals too would be to refactor anything that needs it since the project
is nearing its final form

Also start adding unit tests to the files


need to be able to create a new playlist
rename playlist
delete playlist
add song to playlist
remove song from playlist
play playlist
Need to implement a current song structure
If a new track starts playing from the queue we need to update it
Ways I could implement,

I could ignore the inbuilt queue from rodio and build my own queue to
avoid the queues diverging
This option become more enticing due to the empty function that can
return true when the sink is empty, prompting a new song to go by grabbing
from the queue

I could use the queue since it provides a lot of good features and
make a failsafe option that always mimics the real queue

I could look into rodio::queue and see if that solves any of my issues
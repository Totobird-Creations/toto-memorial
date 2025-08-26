const video_controls = document.getElementsByClassName("video_control");
for (let i = 0; i < video_controls.length; i++) {
    const video_control = video_controls[i];
    video_control.removeAttribute("hidden");

    const video = video_control.parentElement.getElementsByTagName("video")[0];
    video.removeAttribute("controls");

    const control_play = video_control.getElementsByClassName("video_control_play")[0];
    control_play.addEventListener("click", () => {
        control_play.classList.remove("video_control_play_restart");
        if (control_play.checked) {
            video.play();
        } else {
            video.pause();
        }
    });
    video.addEventListener("click", () => {
        if (control_play.checked) {
            control_play.checked = false;
            video.pause();
        } else {
            control_play.checked = true;
            video.play();
        }
    });

    const control_seek  = video_control.getElementsByClassName("video_control_seek")[0];
    const seek_progress = control_seek.getElementsByClassName("video_control_seek_progress")[0];
    const seek_hover    = control_seek.getElementsByClassName("video_control_seek_hover")[0];
    let currently_seeking   = false;
    let playing_before_seek = false;

    const update_seek_frac = (e) => {
        const rect = control_seek.getBoundingClientRect();
        const frac = Math.min(Math.max((e.clientX - rect.left) / rect.width, 0.0), 1.0);
        seek_hover.style.width = (frac * 100.0) + "%";
        if (currently_seeking) {
            video.currentTime = video.duration * frac;
        }
    };

    video.addEventListener("ended", () => {
        if (! currently_seeking) {
            control_play.checked = false;
            control_play.classList.add("video_control_play_restart");
        }
    });

    control_seek.addEventListener("mousedown", (e) => {
        if (! currently_seeking) {
            currently_seeking   = true;
            playing_before_seek = control_play.checked;
            control_play.classList.remove("video_control_play_restart");
            control_play.checked = false;
            video.pause();
            update_seek_frac(e);
        }
    });
    window.addEventListener("mouseup", (e) => {
        if (currently_seeking) {
            update_seek_frac(e);
            currently_seeking = false;
            if (playing_before_seek && video.currentTime < video.duration) {
                control_play.checked = true;
                video.play();
            }
        }
    });

    video.addEventListener("timeupdate", () => {
        seek_progress.style.width = ((video.currentTime / video.duration) * 100.0) + "%";
    });

    document.addEventListener("mousemove", (e) => {
        update_seek_frac(e);
    });

}

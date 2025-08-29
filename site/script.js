// Video controls.
const video_controls = document.getElementsByClassName("video_control");
for (let i = 0; i < video_controls.length; i++) {
    const video_control = video_controls[i];
    video_control.removeAttribute("hidden");

    const video = video_control.parentElement.getElementsByTagName("video")[0];
    video.removeAttribute("controls");

    const control_play = video_control.getElementsByClassName("video_control_play")[0];
    control_play.addEventListener("click", () => {
        if (video.paused) {
            video.play();
        } else {
            video.pause();
        }
    });
    video.addEventListener("pause", () => {
        control_play.checked = false;
    });
    video.addEventListener("play", () => {
        control_play.checked = true;
    });
    video.addEventListener("click", () => {
        if (video.paused) {
            video.play();
        } else {
            video.pause();
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
            control_play.classList.add("video_control_play_restart");
        }
    });

    control_seek.addEventListener("mousedown", (e) => {
        if (! currently_seeking) {
            currently_seeking   = true;
            playing_before_seek = control_play.checked;
            control_play.classList.remove("video_control_play_restart");
            video.pause();
            update_seek_frac(e);
        }
    });
    window.addEventListener("mouseup", (e) => {
        if (currently_seeking) {
            update_seek_frac(e);
            currently_seeking = false;
            if (playing_before_seek && video.currentTime < video.duration) {
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


// Pause/restart video when selecting another entry in the group.
const select_options = document.getElementsByClassName("img_group_select_option");
for (let i = 0; i < select_options.length; i++) {
    const select_option = select_options[i];
    const input         = select_option.getElementsByTagName("input")[0];
    input.addEventListener("input", () => {
        const videos = select_option.parent.getElementsByTagName("video");
        if (input.checked) {
            for (let j = 0; j < videos.length; j++) {
                videos[j].currentTime = 0.0;
            }
        } else {
            for (let j = 0; j < videos.length; j++) {
                videos[j].pause();
            }
        }
    })
}

// Pause/restart video and select cover image when opening/closing group.
const img_wrappers = document.getElementsByClassName("img_wrapper");
for (let i = 0; i < img_wrappers.length; i++) {
    const img_wrapper = img_wrappers[i];
    for (let j = 0; j < img_wrapper.children.length; j++) {
        const child = img_wrapper.children[j];
        if (child.tagName == "INPUT") {
            child.addEventListener("input", () => {
                const videos = img_wrapper.getElementsByTagName("video");
                if (child.checked) {
                    for (let k = 0; k < videos.length; k++) {
                        videos[k].currentTime = 0.0;
                    }
                } else {
                    for (let k = 0; k < videos.length; k++) {
                        videos[k].pause();
                    }
                    setTimeout(() => {
                        img_wrapper.getElementsByClassName("select_option_default")[0].checked = true;
                    }, 250);
                }
            });
        }
    }
}

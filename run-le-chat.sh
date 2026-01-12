#!/bin/bash
# Le Chat Wrapper Script - Audio, permissions setup

# Set audio and media environment variables for optimal WebKit/WebRTC performance
export PULSE_PROP_OVERRIDE="filter.want=echo-cancel"
export GST_PLUGIN_PATH="/usr/lib/x86_64-linux-gnu/gstreamer-1.0:$GST_PLUGIN_PATH"
export LD_LIBRARY_PATH="/usr/lib/x86_64-linux-gnu:/usr/lib:$LD_LIBRARY_PATH"

# Enable WebRTC for audio capture
export WEBRTC_AUDIO_PROCESSING=1

# Set window manager integration hints
export GTK_CSD=0
export XDG_CURRENT_DESKTOP=KDE

# Run the app
exec "/home/christian/Desktop/Le_Chat_App/target/release/mistral-chat-wrapper" "$@"

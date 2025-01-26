<script>
import IconPlay from './icons/IconPlay.vue'
import IconPause from './icons/IconPause.vue'
import IconVolume from './icons/IconVolume.vue'
import IconMute from './icons/IconMute.vue'
import IconClose from './icons/IconClose.vue'
import { useAudioStore } from '../stores/audio'

export default {
  name: 'AudioPlayer',
  components: {
    IconPlay,
    IconPause,
    IconVolume,
    IconMute,
    IconClose,
  },
  props: {
    feedTitle: {
      type: String,
      required: true,
    },
    feedLink: {
      type: String,
      required: true,
    },
    file: {
      type: String,
      required: true,
    },
    title: {
      type: String,
      required: true,
    },
    url: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      audio: undefined,
      currentSeconds: 0,
      durationSeconds: 0,
      innerLoop: false,
      loaded: false,
      playing: false,
      previousVolume: 35,
      volume: 100,
      autoPlay: true,
    }
  },
  computed: {
    muted() {
      return this.volume / 100 === 0
    },
    percentComplete() {
      return (this.currentSeconds / this.durationSeconds) * 100
    },
  },
  watch: {
    playing(value) {
      if (value) {
        return this.audio.play()
      }
      this.audio.pause()
    },
    volume() {
      this.audio.volume = this.volume / 100
    },
  },
  created() {
    this.innerLoop = this.loop
    window.addEventListener('keydown', (event) => {
      switch (event.code) {
        case 'Space':
          this.togglePlay()
          break
        case 'Enter':
          this.togglePlay()
          break
        case 'ArrowUp':
          if (this.volume < 100) this.volume++
          break
        case 'ArrowDown':
          if (this.volume > 0) this.volume--
          break
        case 'ArrowLeft':
          this.goBack15()
          break
        case 'ArrowRight':
          this.goAhead15()
          break
      }
    })
  },
  mounted() {
    this.audio = this.$refs.audioFile
    this.audio.addEventListener('timeupdate', this.update)
    this.audio.addEventListener('loadeddata', this.load)
    this.audio.addEventListener('pause', () => {
      this.playing = false
    })
    this.audio.addEventListener('play', () => {
      this.playing = true
    })
    this.audioStore = useAudioStore()
  },
  methods: {
    convertTimeHHMMSS(val) {
      const hhmmss = new Date(val * 1000).toISOString().substr(11, 8)
      return hhmmss.indexOf('00:') === 0 ? hhmmss.substr(3) : hhmmss
    },
    load() {
      if (this.audio.readyState >= 2) {
        this.loaded = true
        this.durationSeconds = parseInt(this.audio.duration)
        if (this.playing) {
          this.stop()
        }
        this.playing = true
        this.audio.play()
        return this.playing
      }
      throw new Error('Failed to load sound file.')
    },
    mute() {
      if (this.muted) {
        this.volume = this.previousVolume
        return this.volume
      }
      this.previousVolume = this.volume
      this.volume = 0
    },
    seek(e) {
      if (!this.loaded) return
      const el = e.target.getBoundingClientRect()
      const seekPos = (e.clientX - el.left) / el.width
      this.audio.currentTime = this.audio.duration * seekPos
    },
    stop() {
      this.playing = false
      this.audio.currentTime = 0
    },
    togglePlay() {
      this.playing = !this.playing
    },
    update() {
      this.currentSeconds = this.audio.currentTime
    },
    close() {
      this.stop()
      this.audioStore.hideAudioPlayer()
    },
  },
}
</script>

<template>
  <div id="audio-player">
    <audio ref="audioFile" :loop="innerLoop" :src="file" preload="auto" style="display: none" />

    <div id="player">
      <div id="player-controls">
        <a class="player-icon" :aria-label="playing ? 'pause' : 'play'" @click="togglePlay">
          <IconPlay v-if="!playing" />
          <IconPause v-if="playing" />
        </a>
        <div id="player-track">
          <div id="player-track-desc">
            <span v-if="feedLink">
              <a id="player-track-feed" :href="feedLink"> {{ feedTitle }}</a>
              <span> - </span>
            </span>
            <a :href="url" id="player-track-title"> {{ title }}</a>
          </div>

          <div id="player-track-progress" @click.prevent="seek">
            <div :style="{ width: percentComplete + '%' }" class="player-track-seeker" />
          </div>

          <div id="player-track-time">
            <span id="player-track-time-current">{{ convertTimeHHMMSS(currentSeconds) }}</span>
            <span id="player-track-time-separator">/</span>
            <span id="player-track-time-total">{{ convertTimeHHMMSS(durationSeconds) }}</span>
          </div>
        </div>
        <div id="player-additional-controls">
          <a
            tabindex="0"
            class="player-icon"
            id="audio-player-mute-button"
            :aria-label="muted ? 'unmute' : 'mute'"
            @click="mute"
            @keypress.space.enter="mute"
          >
            <IconVolume v-if="!muted" />
            <IconMute v-if="muted" />
          </a>
          <a tabindex="1" class="player-icon" @click="close" id="audio-player-close-button">
            <IconClose />
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
#audio-player {
  position: fixed;
  width: 100vw;
  height: 80px;
  bottom: 0;
  left: 0;
  background-color: var(--color-background);
  z-index: 999;
  border-top: 1px solid rgb(from var(--color-accent) r g b / 80%);
}
#player {
  padding: 0.85rem;
  font-weight: 300;
  position: relative;
}

#player-controls {
  display: flex;
  align-items: center;
}

.player-icon {
  width: 24px;
  height: 24px;
  display: flex;
  cursor: pointer;
  margin-left: 10px;
}

#player-track {
  flex: auto;
  padding: 0 2rem;
  overflow: hidden;
  width: 100%;
}

#player-track-contents {
  font-weight: 300;
  text-overflow: ellipsis;
  overflow: hidden;
  white-space: nowrap;
}
#player-track-feed {
  opacity: 0.6;
}

#player-track-progress {
  position: absolute;
  background-color: var(--color-accent);
  cursor: pointer;
  min-width: 200px;
  top: 0;
  left: 0;
  right: 0;
  height: 5px;
}

#player-track-progress .player-track-seeker {
  background-color: var(--color-highlight);
  bottom: 0;
  left: 0;
  position: absolute;
  top: 0;
  z-index: 20;
}

#player-track-time {
  display: flex;
}

#player-track-time-current {
  opacity: 1;
  margin-right: 0.25rem;
}

#player-track-time-separator {
  opacity: 0.6;
}

#player-track-time-total {
  opacity: 0.6;
  margin-left: 0.25rem;
}

#audio-player-mute-button {
  display: none;
}

@media (min-width: 768px) {
  #audio-player {
    padding: 0 20%;
  }
  #player-additional-controls {
    display: flex;
    justify-content: flex-end;
  }
  #player-track-time {
    justify-content: flex-end;
  }
  #audio-player-mute-button {
    display: block;
  }
  #player-track-progress {
    top: -5px;
    height: 3px;
    margin-top: 0.75rem;
    position: relative;
  }
}
</style>

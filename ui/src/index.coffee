import { User } from './Login.js'

root = document.body
#root.onclick = ->
#    audio = new Audio('audio_file.mp3')
#    audio.play()

MyComponent = ->
    view: ->
        <main>
            <h1 class="title">My first app</h1>
            <button>a button</button>
            <User/>
            <a href="https://reddit.com">don't go here plz!</a>
        </main>

startup = -> m.mount(root, MyComponent)

SomeButton = ->
    view: -> <button onclick={startup}>Enter the Dungeon</button>

m.mount(root, SomeButton)










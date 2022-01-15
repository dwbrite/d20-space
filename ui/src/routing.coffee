import { Login } from './login.js'
#import { Login } from './login.js'
#import { Login } from './login.js'

Test =
  view: ->
    <h1>"bepsi"</h1>

Home =
  view: ->
    if Login.logged_in then <Test/> else <Login/>

m.route.prefix = ""
m.route(document.body, "/", {
    "/": Login
    "/home": Home
  }
)

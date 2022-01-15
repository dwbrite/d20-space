export Extra =
  loadLoginCreds: ->
    return if !document.cookie
    cookies = document.cookie.split('; ')

    user = cookies.find((a) -> return a.startsWith('user='))
    pin = cookies.find((a) -> return a.startsWith('pin='))

    Login.creds.user = if user then user.split('=')[1] else null
    Login.creds.pin = if pin then pin.split('=')[1] else null
  saveLoginCreds: ->
    document.cookie = "user=" + Login.creds.user + "; SameSite=Strict"
    document.cookie = "pin=" + Login.creds.pin + "; SameSite=Strict"

User =
  view: (vnode) -> [
    <div class={"user"} onclick={this.onclick(vnode)}>
      <img class={"user-icon"} src={"/static/blob_woah.png"}>icon</img>
      <div class={"user-name"}>{vnode.attrs.name}</div>
    </div>
  ]
  onclick: (vnode) -> ->
    Login.creds.user = vnode.attrs.name

UserList =
  users: []
  loadList: ->
    return m.request(
      method: "GET"
      url: "http://localhost:3000/api/v0/users"
      withCredentials: false
    ).then((result) ->
      UserList.users = result.users
    )

Title =
  view: -> [
    "Who Cometh",
    <br/>,
    "to the Land of Estar?"
  ]

SelectUserComponent =
  view: -> [
    <div class={"login-select-user"}>
      <div class={"login-title"}><Title/></div>
      {<User name={user.name}/> for user in UserList.users}
    </div>
  ]

EnterPinComponent =
  view: -> [
    <div class={"login-enter-pin"}>
      <div class={"login-title"}>
        <button onclick={->
          Login.creds.user = null
          Login.creds.pin = null
        }>Back</button>

        {Login.creds.user}
      </div>
      <input type={"text"} oninput={(ev) -> Login.creds.pin = ev.target.value} value={Login.creds.pin}/>
      <button onclick={EnterPinComponent.log_in}>Submit</button>
    </div>
  ]
  log_in: ->
    console.log("o hej", Login.creds.user, Login.creds.pin)
    options =
    m.request(options =
      method: "POST"
      url: "/api/v0/users/authenticate"
      headers: Authorization: "Basic " + btoa(Login.creds.user+":"+Login.creds.pin)
    ).then((result) ->
      console.log("Logging in as ", Login.creds.user, "...")
      Login.logged_in = true
      Extra.saveLoginCreds()
    , ->
    )


export Login =
  logged_in: false
  creds:
    user: null
    pin: null
  view: -> [
    if !this.logged_in
      <div class={"modal-backdrop"}>
        <div class={"login-modal"}>{
            if this.creds.user?
              <EnterPinComponent/>
            else
              <SelectUserComponent/>
        }</div>
      </div>
  ]
  oninit: ->
    Extra.loadLoginCreds()
  oncreate: ->
    UserList.loadList()

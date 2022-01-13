UserComponent =
  view: (vnode) ->
    <div class={"user"}>
      <img class={"user-icon"} src={"/blob_woah.png"}>icon</img>
      <div class={"user-name"}>{vnode.attrs.name}</div>
    </div>

appdata = {
  users: []
}

UsersModel =
  loadList: ->
    return m.request(
      method: "GET"
      url: "http://localhost:3000/api/v0/users"
      withCredentials: false
    ).then((result) ->
      appdata = result
    )

export User =
  view: -> [
    <div class={"login"}>
      <div class={"modal"}>
        {appdata.users.map((item) -> return <UserComponent name={item.name}/>)}
      </div>
    </div>
  ]
  oninit: ->
    console.log("fuck you?")
    UsersModel.loadList()
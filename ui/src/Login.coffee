UserComponent =
  view: (vnode) ->
    <div class={"user"}>
      <img class={"user-icon"} src={"/blob_woah.png"}>icon</img>
      <div class={"user-name"}>{vnode.attrs.name}</div>
    </div>

UsersModel =
  users: []
  loadList: ->
    return m.request(
      method: "GET"
      url: "http://localhost:3000/api/v0/users"
      withCredentials: false
    ).then((result) ->
      UsersModel.users = result.users
    )

Title =
  view: -> [
    "Who Cometh",
    <br/>,
    "to the Land of Estar?"
  ]

export Login =
  view: -> [
    <div class={"login"}>
      <div class={"login-modal"}>
        <div class={"login-title"}><Title/></div>
        {<UserComponent name={user.name}/> for user in UsersModel.users}
        {<UserComponent name={user.name}/> for user in UsersModel.users}
        {<UserComponent name={user.name}/> for user in UsersModel.users}
      </div>
    </div>
  ]

  oninit: ->
    UsersModel.loadList()
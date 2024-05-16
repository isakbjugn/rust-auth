<script>
  import {BASE_API_URI} from "$lib/constants";

  let user = {
    id: '',
    first_name: '',
    last_name: '',
  }
    // fetch user from BASE_URL/users
    fetch(`${BASE_API_URI}/users`, {
        method: 'GET',
        headers: {
            Accept: 'application/json',
        },
        credentials: 'include',
    }).then(async (response) => {
        if (response.ok) {
            user = await response.json();
            console.log("Response: ", response);
        } else {
            console.error(response);
        }
    }).catch((error) => {
        console.error(error);
    });
</script>

<div>
    <h1>User</h1>
    <p>{user.first_name} {user.last_name}</p>
</div>
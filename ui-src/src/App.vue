<template>
  <LoginModal/>
  <MapView/>
</template>

<script>
import MapView from './components/MapView.vue'
import LoginModal from './components/LoginModal.vue'

export default {
  name: 'App',
  components: {
    LoginModal,
    MapView
  }
}
</script>

<style>
#app {
  width: 100%;
  height: 100%;
}
</style>

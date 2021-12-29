<template>
  <MapView msg="Welcome to Your Vue.js App"/>
</template>

<script>
import MapView from './components/MapView.vue'

export default {
  name: 'App',
  components: {
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

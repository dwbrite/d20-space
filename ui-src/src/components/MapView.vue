<template>
  <canvas id="map_view"></canvas>
</template>

<script>
export default {
  name: 'MapView',
  mounted() {
    var canvas = document.getElementById('map_view'),
        context = canvas.getContext('2d');

    canvas.style.width = screen.width;
    canvas.style.height = screen.height;
    canvas.width = screen.width;
    canvas.height = screen.height;

    function resizeCanvas() {
      console.log(window.innerHeight);
      make_base();
    }

    new ResizeObserver(resizeCanvas).observe(canvas);


    make_base();

    function make_base()
    {
      let base_image = new Image();
      let scale = 0.5;
      base_image.src = '/temp-estar.jpg';
      base_image.onload = function(){
        context.drawImage(base_image, 0, 0, base_image.width * scale, base_image.height * scale);
        console.log(screen.width, screen.height);
      }
    }
  }
}

</script>

<style>
#map_view {
  display: block;
  position: fixed;
}
</style>

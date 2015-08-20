(function (){

    var socket = new WebSocket("ws://" + location.hostname + ":8010", "rust-websocket");

    socket.onmessage = function (event) {
        eval(event.data);
    };

    // rendering part

    // create the scene and camera
    var scene = new THREE.Scene();
    var camera = new THREE.PerspectiveCamera( 75, window.innerWidth/window.innerHeight, 0.1, 1000 );

    // create renderer and element
    var renderer = new THREE.WebGLRenderer();
    renderer.setSize( window.innerWidth, window.innerHeight );
    document.body.appendChild( renderer.domElement );

    // generate init geometry
    // some lighting
    var directionalLight = new THREE.DirectionalLight( 0xffffff, 1.0 );
    directionalLight.position.set( 0, 1, 0 );
    scene.add( directionalLight );

    var light = new THREE.AmbientLight( 0x404040 ); // soft white light
    scene.add( light );

    // lathe
    var SEGMENTS = 20;

    var points = [];
    var lathe_geom = new THREE.LatheGeometry( points, SEGMENTS );
    lathe_geom.dynamic = true;

    var material = new THREE.MeshPhongMaterial( { color: 0xffffff, specular: 0x000000, shininess: 30 } );

    var lathe = new THREE.Mesh( lathe_geom, material );
    lathe.rotation.x = Math.PI;
    scene.add( lathe );
    
    var lathe_rotate = new THREE.Mesh( lathe_geom, material );
    lathe_rotate.rotation.x = Math.PI;
    lathe_rotate.position.z -= 100;
    scene.add( lathe_rotate );

    camera.position.z = 100;

    var render = function () {
        requestAnimationFrame( render );

        // so the server only emits when clients connected
        if ( socket != null && socket.readyState == 1 ) {
            socket.send("emit");
        }

        lathe.geometry = new THREE.LatheGeometry( points, SEGMENTS );
        lathe_rotate.geometry = new THREE.LatheGeometry( points, SEGMENTS );

        lathe_rotate.rotation.x += 0.01;

        renderer.render(scene, camera);
    };

    render();

})();

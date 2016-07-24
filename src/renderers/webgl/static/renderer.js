(function() {
  // Constants
  var UpdateViewerTransformMessage = 0;
  var AddCameraMessage = 1;
  var DeleteCameraWithNameMessage = 2;
  var UpdateCameraTransformMessage = 3;
  var UpdateCameraProjectionMessage = 4;
  var SetInitialStateMessage = 5;

  var socket = new WebSocket('ws://localhost:12345');

  socket.onopen = function() {
    console.log('WebSocket opened...');
  };

  socket.onmessage = function(message) {
    var data = JSON.parse(message.data);
    switch (data.code) {
    case SetInitialStateMessage:
    }
  };
})();

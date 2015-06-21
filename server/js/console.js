(function() {
  var ws = new WebSocket("ws://localhost:8000/echo");
  ws.onopen = function() {
    ws.send("socket open");
  };
  ws.onclose = function(evt) {
    alert("socket closed");
  };
  ws.onmessage = function(msg) {
    console.log(msg);
  }
})();

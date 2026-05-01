<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Ромб</title>
</head>
<body>
<pre id="diamond" style="font-family: monospace; font-size: 16px; line-height: 1.2;"></pre>
<script>

  const HEIGHT = 11;
  const SYMBOL = '*';

  let output = "";


  const maxWidth = HEIGHT;

  for (let row = 0; row < HEIGHT; row++) {

    let stars;
    if (row <= Math.floor(HEIGHT / 2)) {
      stars = 2 * row + 1;
    } else {
      stars = 2 * (HEIGHT - row - 1) + 1;
    }
    const spaces = (maxWidth - stars) / 2;


    let line = " ".repeat(spaces) + SYMBOL.repeat(stars);
    output += line + "\n";
  }

  
  console.log(output);
  document.getElementById("diamond").textContent = output;
</script>
</body>
</html>

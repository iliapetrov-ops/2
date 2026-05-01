<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Ялинка</title>
</head>
<body>
<pre id="tree" style="font-family: monospace; font-size: 160px; background: #f4f4f4; padding: 100px;"></pre>
<script>
  
  const TRIANGLES_COUNT = 6;
  const maxWidth = 2 * TRIANGLES_COUNT - 1;
  let output = "";
  Array.from({ length: TRIANGLES_COUNT }, (_, i) => i + 1).forEach(height => {
   
    Array.from({ length: height }, (_, i) => i).forEach(row => {
      const stars = 2 * row + 1;           
      const spaces = (maxWidth - stars) / 2; 
      output += " ".repeat(spaces) + "*".repeat(stars) + "\n";
    });
  });
  console.log(output);
  document.getElementById("tree").textContent = output;
</script>
</body>
</html>

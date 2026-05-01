<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Найбільший спільний дільник (GCD)</title>
</head>
<body>
<pre id="gcdResult" style="font-family: monospace; font-size: 16px; background: #f4f4f4; padding: 10px;"></pre>
<script>
  const A = 48;
  const B = 18;
  function gcd(a, b) {
    
    a = Math.abs(a);
    b = Math.abs(b);
    
    while (b !== 0) {
      let temp = b;
      b = a % b;
      a = temp;
    }
    return a;
  }
  const result = gcd(A, B);
  const output = `Число A: ${A}\nЧисло B: ${B}\nGCD (НСД): ${result}`;
  console.log(output);
  document.getElementById("gcdResult").textContent = output;
</script>
</body>
</html>

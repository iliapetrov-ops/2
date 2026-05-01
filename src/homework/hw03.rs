<!DOCTYPE html>
<html lang="uk">
<head>
    <meta charset="UTF-8">
    <title>Конверт</title>
</head>
<body>
<pre id="envelope" style="font-family: monospace; font-size: 16px; line-height: 1.2;"></pre>
<script>

    const WIDTH = 30;
    const HEIGHT = 15;

    let output = "";


    output += "*".repeat(WIDTH) + "\n";

    const center = Math.floor((HEIGHT - 1) / 2);

    for (let i = 1; i < HEIGHT - 1; i++) {
        const t = Math.min(i, HEIGHT - 1 - i);

        let left = (i <= center) ? 2 * t + 1 : 2 * t;
        let right = WIDTH - 1 - left;


        let line = Array(WIDTH).fill(' ');
        line[0] = '*';               // ліва межа
        line[WIDTH - 1] = '*';       // права межа

        if (left <= right) {
            line[left] = '*';
            line[right] = '*';
        } else {
            // Центральний рядок — суцільна смужка зірочок
            for (let pos = right; pos <= left; pos++) {
                line[pos] = '*';
            }
        }

        output += line.join('') + "\n";
    }


    output += "*".repeat(WIDTH);

 
    document.getElementById("envelope").textContent = output;
    console.log(output);
</script>
</body>
</html>

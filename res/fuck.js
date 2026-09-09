const htmlPdf = require('html-pdf-chrome')

const fs = require('node:fs');

fs.readFile('./LoveDota.svg', 'utf8', (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
	const options = {
	    completionTrigger: new htmlPdf.CompletionTrigger.Timer(5000),
	    chromeFlags: ['--headless']
	};

	let pdf = htmlPdf.create(data, options).then((pdf) => pdf.toFile('Eve-CV.pdf'));
});




var head = document.getElementsByTagName("head")[0];

// Global site tag (gtag.js) - Google Analytics with Klaro script
var gtagScript = document.createElement("script");
gtagScript.async = true;
gtagScript.type = "opt-in";
gtagScript.setAttribute("data-type", "application/javascript");
gtagScript.setAttribute(
	"data-src",
	"https://www.googletagmanager.com/gtag/js?id=UA-145158313-2"
);
gtagScript.setAttribute("data-name", "googleAnalytics");
head.appendChild(gtagScript);

var dataLayerScript = document.createElement("script");
dataLayerScript.type = "opt-in";
dataLayerScript.text = `
window.dataLayer = window.dataLayer || [];
function gtag(){
	dataLayer.push(arguments);
}
gtag('js', new Date());
gtag('config', 'UA-145158313-2');
`;
dataLayerScript.setAttribute("data-type", "application/javascript");
dataLayerScript.setAttribute("data-name", "googleAnalytics");
head.appendChild(dataLayerScript);

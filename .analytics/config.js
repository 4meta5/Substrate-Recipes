// By default, Klaro will load the config from  a global "klaroConfig" variable.
// You can change this by specifying the "data-config" attribute on your
// script take, e.g. like this:
// <script src="klaro.js" data-config="myConfigVariableName" />
// You can also disable auto-loading of the consent notice by adding
// data-no-auto-load=true to the script tag.
var klaroConfig = {
	// You can customize the ID of the DIV element that Klaro will create
	// when starting up. If undefined, Klaro will use 'klaro'.
	elementID: 'klaro',

	// How Klaro should store the user's preferences. It can be either 'cookies'
	// or 'localStorage'. If undefined, Klaro will use cookies.
	klaroStorage: 'localStorage',

	// You can customize the name of the cookie that Klaro uses for storing
	// user consent decisions. If undefined, Klaro will use 'klaro'.
	cookieName: 'substrateDevHubCookie',

	// You can also set a custom expiration time for the Klaro cookie.
	// By default, it will expire after 120 days.
	cookieExpiresAfterDays: 120,

	// You can customize the name of the cookie that Klaro will use to
	// store user consent. If undefined, Klaro will use 'klaro'.

	// Put a link to your privacy policy here (relative or absolute).
	privacyPolicy: 'https://www.parity.io/privacy/',

	// Defines the default state for applications (true=enabled by default).
	default: true,

	// If "mustConsent" is set to true, Klaro will directly display the consent
	// manager modal and not allow the user to close it before having actively
	// consented or declines the use of third-party apps.
	mustConsent: false,

	// You can define the UI language directly here. If undefined, Klaro will
	// use the value given in the global "lang" variable. If that does
	// not exist, it will use the value given in the "lang" attribute of your
	// HTML tag. If that also doesn't exist, it will use 'en'.
	//lang: 'en',

	// You can overwrite existing translations and add translations for your
	// app descriptions and purposes. See `src/translations.yml` for a full
	// list of translations that can be overwritten:
	// https://github.com/DPKit/klaro/blob/master/src/translations.yml

	// Example config that shows how to overwrite translations:
	// https://github.com/DPKit/klaro/blob/master/src/configs/i18n.js
	translations: {
		// If you erase the "consentModal" translations, Klaro will use the
		// defaults as defined in translations.yml
		en: {
			consentModal: {
				description:
					'We use necessary cookies to make our site work. We\'d also like to set optional tracking mechanisms to help us improve it by collecting and reporting information on how you use it. Here you can customize the information we collect about you by giving or revoking your consent to our use of optional tracking mechanisms.',
			},
			googleAnalytics: {
				description: 'Collection of information about how visitors use our website',
			},
			cloudflare: {
				description: 'Protection against DDoS attacks',
			},
			purposes: {
				analytics: 'analytics and improvement of our sites',
				security: 'security',
			},
			close: "Save settings"
		},
	},

	// This is a list of third-party apps that Klaro will manage for you.
	apps: [
		{
			// Each app should have a unique (and short) name.
			name: 'googleAnalytics',

			// If "default" is set to true, the app will be enabled by default
			// Overwrites global "default" setting.
			// We recommend leaving this to "false" for apps that collect
			// personal information.
			default: true,

			// The title of you app as listed in the consent modal.
			title: 'Google Analytics',

			// The purpose(s) of this app. Will be listed on the consent notice.
			// Do not forget to add translations for all purposes you list here.
			purposes: ['analytics'],

			// A list of regex expressions or strings giving the names of
			// cookies set by this app. If the user withdraws consent for a
			// given app, Klaro will then automatically delete all matching
			// cookies.
			cookies: [
				// you can also explicitly provide a path and a domain for
				// a given cookie. This is necessary if you have apps that
				// set cookies for a path that is not "/" or a domain that
				// is not the current domain. If you do not set these values
				// properly, the cookie can't be deleted by Klaro
				// (there is no way to access the path or domain of a cookie in JS)
				['_ga', '_gat_gtag_UA-145158313-2', '_gid'], //for the production version
			],

			// If "required" is set to true, Klaro will not allow this app to
			// be disabled by the user.
			required: false,

			// If "optOut" is set to true, Klaro will load this app even before
			// the user gave explicit consent.
			// We recommend always leaving this "false".
			optOut: false,

			// If "onlyOnce" is set to true, the app will only be executed
			// once regardless how often the user toggles it on and off.
			onlyOnce: false,
		},

		// The apps will appear in the modal in the same order as defined here.
		{
			name: 'cloudflare',
			title: 'Cloudflare',
			purposes: ['security'],
			required: true,
		},
	],
};

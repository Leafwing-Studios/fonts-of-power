const autofocusPlugin = (hook, vm) => {
	hook.ready(function () {
		const search = window.Docsify.dom.find('.search input');
		
		search.autofocus = true;
		search.focus();
	});
}

window.$docsify.plugins = [].concat(autofocusPlugin, window.$docsify.plugins)

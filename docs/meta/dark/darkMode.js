const plugin = (hook, vm) => {
  var trans = () => {
    document.documentElement.classList.add('transition')
    window.setTimeout(() => {
      document.documentElement.classList.remove('transition')
    }, 210)
  }
	
  var setColor = ({ background, backgroundHighlight, uiHighlight, primaryContent, emphasizedContent, themeColor }) => {
		var splitBackground = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(background)
		var splitBackground = splitBackground || ['', 'ff', 'ff', 'ff']
		
    document.documentElement.style.setProperty('--docsify_dark_mode_bg', background)
    document.documentElement.style.setProperty('--ui_highlight', uiHighlight)
    document.documentElement.style.setProperty('--primary_content', primaryContent)
		document.documentElement.style.setProperty('--emphasized_content', emphasizedContent)
    document.documentElement.style.setProperty('--background_highlight', backgroundHighlight)
    document.documentElement.style.setProperty('--theme-color', themeColor)
		document.documentElement.style.setProperty(
			'--docsify_dark_mode_bg_rgb', 
			`${parseInt(splitBackground[1], 16)}, ${parseInt(splitBackground[2], 16)}, ${parseInt(splitBackground[3], 16)}`
		)
  }

  var defaultConfig = {
    dark: {
      background: '#002b36',
      backgroundHighlight: '#073642',
			uiHighlight: '#586e75',
			primaryContent: '#eee8d5',
      emphasizedContent: '#fdf6e3',
      themeColor: '#2aa198',
    },
    light: {
      background: '#fdf6e3',
			backgroundHighlight: '#eee8d5',
			uiHighlight: '#93a1a1',
      primaryContent: '#073642',
      emphasizedContent: '#002b36',
      themeColor: '#2aa198',
    }
  }

  const theme = {
    dark:{
      ...defaultConfig.dark, ...vm.config.darkMode.dark
    },
    light: {
      ...defaultConfig.light, ...vm.config.darkMode.light
    }
  }
	
	hook.mounted(function () {
		var checked = ''
		if (localStorage.getItem('DOCSIFY_DARK_MODE')) {
			currColor = localStorage.getItem('DOCSIFY_DARK_MODE');
			if (currColor === 'dark') checked = 'checked'
		}
		
    var darkEl = ` <div id="dark_mode">
     <input class="container_toggle" type="checkbox" id="switch" name="mode" ${checked} />
     <label for="switch">Toggle</label>
   </div>`
		
		const el = window.Docsify.dom.create('div', darkEl);
		const aside = window.Docsify.dom.find('aside');
		window.Docsify.dom.appendTo(aside, el);
	});

  hook.doneEach(function() {
    var currColor
    if (localStorage.getItem('DOCSIFY_DARK_MODE')) {
      currColor = localStorage.getItem('DOCSIFY_DARK_MODE')
      setColor(theme[`${currColor}`])
    } else {
      currColor = 'light'
      setColor(theme.light)
    }

    var checkbox = document.querySelector('input[name=mode]')
    
    if (!checkbox) {
      return
    }

    checkbox.addEventListener('change', function() {
      // dark
      if (currColor === 'light') {
        trans()
        setColor(theme.dark)
        localStorage.setItem('DOCSIFY_DARK_MODE', 'dark')
        currColor = 'dark'
      } else {
        trans()
        setColor(theme.light)
        localStorage.setItem('DOCSIFY_DARK_MODE', 'light')
        currColor = 'light'
      }
    })
  })
}

window.$docsify.plugins = [].concat(plugin, window.$docsify.plugins)

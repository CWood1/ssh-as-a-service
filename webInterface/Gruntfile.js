module.exports = function(grunt) {
    // Project configuration.
    grunt.initConfig({
	pkg: grunt.file.readJSON('package.json'),
	'browserify': {
	    options: {
		debug: true,
		transform: [['babelify', {'presets': ['es2015', 'react']}]],
		extensions: ['.js'],
	    },
	    dev: {
		options: {
		    alias: ['react:']  // Make React available externally for dev tools
		},
		src: ['index.js'],
		dest: '../webDeploy/bundle.js'
	    },
	    production: {
		options: {
		    debug: false
		},
		src: '<%= browserify.dev.src %>',
		dest: 'build/bundle.js'
	    }
	}
    });

    // Load the plugin that provides the "uglify" task.
    grunt.loadNpmTasks('grunt-browserify');

    // Default task(s).
    grunt.registerTask('default', ['browserify']);

};

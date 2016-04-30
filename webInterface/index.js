import { Router, Route, Link, browserHistory } from 'react-router'
import React from 'react'
import { render } from 'react-dom'

const test = React.createClass({
    render() {
	return (
		<h1>Test</h1>
	)
    }
})

render((
	<Router history={browserHistory}>
	  <Route path="*" component={test}>
	  </Route>
	</Router>
), document.getElementById('page'))

import { Router, Route, Link, browserHistory } from 'react-router'
import React from 'react'
import { render } from 'react-dom'

const machines = {
    "10.0.0.1": false,
    "4.2.2.2": true,
    "example.com": true
};

const test = React.createClass({
    render() {
	var machinesContent = [];
	var host;
	
	for(host in machines) {
	    machinesContent.push(
		    <div className="col-md-2" key={host}>
 		    <h2 style={{"color": machines[host] ? "green" : "red", "textAlign": "center"}}>
		    <span className="glyphicon glyphicon-hdd"></span>
		    <br />
   		        {host}
		      </h2>
		    </div>
	    );
	}
	
	return (
	    <div className="container">
		<div className="row">
 	  	  {machinesContent}
	        </div>
	    </div>
	)
    }
})

render((
	<Router history={browserHistory}>
	  <Route path="*" component={test}>
	  </Route>
	</Router>
), document.getElementById('page'))

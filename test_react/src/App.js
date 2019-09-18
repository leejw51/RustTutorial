import React from 'react'
import logo from './logo.svg'
import './App.css'
import Button from '@material-ui/core/Button'
import TextField from '@material-ui/core/TextField'
import Paper from '@material-ui/core/Paper'
import Typography from '@material-ui/core/Typography'
function App () {
  return (
    <div>
      <Paper>
        <Typography variant='h5' component='h3'>
          This is a sheet of paper.
        </Typography>
        <Typography component='p'>
          Paper can be used to build surface or other elements for your
          application.
        </Typography>
        <TextField id='standard-name' label='Name' margin='normal' /> <br />
        <Button variant='contained' color='primary'>
          Hello, hello Can you hear me, as I scream your name Hello, hello Do
          you need me, before I fade away
        </Button>
      </Paper>
    </div>
  )
}

export default App

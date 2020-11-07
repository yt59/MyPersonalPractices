const chalk = require('chalk')
const yargs = require('yargs')
const notes = require('./notes.js')

yargs.version('1.1.0')

yargs.command({
    command: 'add',
    describe: 'Adding some note!',
    builder: {
        title: {
            discribe: 'name of note.',
            demandOption: true,
            type: 'string',
        },
        body: {
            discribe: 'body of note.',
            demandOption: true,
            type: 'string',
        },
    },
    handler: function (argv) {
        notes.addNote(argv.title, argv.body)
        console.log('new note saved.')
    }
})

yargs.command({
    command: 'remove',
    describe: 'Removing some note!',
    builder: {
        title: {
            discribe: 'name of note.',
            demandOption: true,
            type: 'string',
        },
    },
    handler: function (argv) {
        notes.removeNote(argv.title)
    }
})

yargs.command({
    command: 'list',
    describe: 'List all notes',
    handler: function () {
        notes.list().forEach(note => {
            console.log(note.title+': '+note.body)
        })
    }
})

yargs.parse()
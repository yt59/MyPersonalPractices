const fs = require('fs')

const addNote = function(title, body){
    notes = loadNotes()
    duplications = notes.filter(function(note){
        return (note.title === title)
    })
    if (duplications.length === 0) {
        notes.push({
            title: title,
            body: body
        })
        saveNotes(notes)
    } else {
        console.log('title has taken!')
    }
}

const removeNote = function(title){
    notes = loadNotes()
    survivers = notes.filter(function(note){
        return !(note.title===title)
    })
    console.log(notes.length>survivers.length?'Note Removd!': 'NOT FIND!')
    saveNotes(survivers)
}

const saveNotes = function(notes){
    fs.writeFileSync('notes.json', JSON.stringify(notes))
}

const loadNotes = function(){
    try {
        data = fs.readFileSync('notes.json')
        return JSON.parse(data.toString())
    } catch (e) {
        return []
    }
}

const list = function(){
    return loadNotes()
}

module.exports = {
    addNote: addNote,
    list: list,
    removeNote: removeNote,
    // showNote: showNote
}
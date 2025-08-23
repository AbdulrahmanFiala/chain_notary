const getLabeledQuarer = (quarterNumber: number) => {
  if (quarterNumber < 1 || quarterNumber > 4) return quarterNumber
    
  const label_quarters: { [key:string]: string} = {
    "1": "1st",
    "2": "2nd",
    "3": "3rd",
    "4": "4th"
  }
  return label_quarters[`${quarterNumber}`]
}

export default getLabeledQuarer;
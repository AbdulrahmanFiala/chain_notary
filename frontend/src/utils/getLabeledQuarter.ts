const getLabeledQuarter = (quarterNumber: number) => {
  const label_quarters: { [key: string]: string } = {
    "1": "1st",
    "2": "2nd",
    "3": "3rd",
    "4": "4th"
  }
  if (`${quarterNumber}` in label_quarters) return label_quarters[`${quarterNumber}`]

  return quarterNumber
}

export default getLabeledQuarter;
// @ts-check

/**
 * Create an appointment
 *
 * @param {number} days
 * @param {number} [now] (ms since the epoch, or undefined)
 *
 * @returns {Date} the appointment
 */
export function createAppointment(days, now = undefined) {
  return now===undefined ?new Date(Date.now()+1000*60*60*24*days):new Date(now+1000*60*60*24*(days)) 
}

/**
 * Generate the appointment timestamp
 *
 * @param {Date} appointmentDate
 *
 * @returns {string} timestamp
 */
export function getAppointmentTimestamp(appointmentDate) {
  return appointmentDate.toISOString();
}

/**
 * Get details of an appointment
 *
 * @param {string} timestamp (ISO 8601)
 *
 * @returns {Record<'year' | 'month' | 'date' | 'hour' | 'minute', number>} the appointment details
 */
export function getAppointmentDetails(timestamp) {
  let d=new Date(timestamp);
  return {year:d.getFullYear(),month: d.getMonth(), date: d.getDate(), hour: d.getHours(), minute: d.getMinutes() }
}

/**
 * Update an appointment with given options
 *
 * @param {string} timestamp (ISO 8601)
 * @param {Partial<Record<'year' | 'month' | 'date' | 'hour' | 'minute', number>>} options
 *
 * @returns {Record<'year' | 'month' | 'date' | 'hour' | 'minute', number>} the appointment details
 */
let i=0;
export function updateAppointment(timestamp, options) {
  if(i===3) console.log(getAppointmentDetails(timestamp),timestamp,options)
  let a=new Date(timestamp);
  for(let key in options){
    if(key=="year") a.setYear(options[key])
    else if(key=="month") a.setMonth(options[key])
    else if(key=="date") a.setDate(options[key])
    else if(key=="hour") a.setHours(options[key])
    else if(key=="minute") a.setMinutes(options[key])
  }
  i++;let d=a;
  return {year:d.getFullYear(),month: d.getMonth(), date: d.getDate(), hour: d.getHours(), minute: d.getMinutes() }
}

/**
 * Get available time in seconds (rounded) between two appointments
 *
 * @param {string} timestampA (ISO 8601)
 * @param {string} timestampB (ISO 8601)
 *
 * @returns {number} amount of seconds (rounded)
 */
export function timeBetween(timestampA, timestampB) {
  return Math.round((new Date(timestampB).getTime() - new Date(timestampA).getTime())/1000 )
}

/**
 * Get available times between two appointment
 *
 * @param {string} appointmentTimestamp (ISO 8601)
 * @param {string} currentTimestamp (ISO 8601)
 */
export function isValid(appointmentTimestamp, currentTimestamp) {
  return new Date(appointmentTimestamp).getTime() >= new Date(currentTimestamp).getTime()
}

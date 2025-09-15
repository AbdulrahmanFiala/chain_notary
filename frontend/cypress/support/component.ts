import './commands'
import { mount } from 'cypress/react'

/* eslint-disable @typescript-eslint/no-namespace */
declare global {
  namespace Cypress {
    interface Chainable {
      mount: typeof mount
    }
  }
}

// @ts-expect-error Cypress namespace not available at runtime 
Cypress.Commands.add('mount', mount)
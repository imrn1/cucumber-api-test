
Feature: /v1/Account/GetPersonalAccount servisi
  


  Scenario Outline: Title of your scenario outline
    Given Get personal account with account number <account_number>
    When send request to <url> with <method> method
    Then Verify <verifyKey> is <expectedValue>
    
    Examples: 
      | account_number | url                            | method | verifyKey   | expectedValue |
      | ESRAYKT        | /v1/Account/GetPersonalAccount | POST   | status      |             0 |
    #  | 126585210200   | /v1/Account/GetPersonalAccount | POST   | status      |             2 |
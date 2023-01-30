Feature: Title of your feature
  I want to use this template for my feature file

  Scenario Outline: Title of your scenario outline
    Given Get wallet info with account number <account_number> and <wallet_number>
    When send request to <url> with <method> method
    Then Verify <verifyKey> is <expectedValue>

    Examples: 
      | account_number  | wallet_number 					| url  						| method 	| verifyKey   | expectedValue |
      | 2638386867 			| 1087615544							| /v1/wallet/info |	POST		|		status		| 			0				|
  #    | 2722802978 			| LYT_8167239205922663437 | /v1/wallet/info | POST		|		status		|				0				|
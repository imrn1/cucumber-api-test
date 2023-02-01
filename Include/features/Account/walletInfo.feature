Feature: Title of your feature
  I want to use this template for my feature file



  Scenario Outline: Title of your scenario outline
    Given Get wallet info with account number <account_number> and <wallet_number>
    When send request to <url> with <method> method
    Then Verify <verifyKey> is <expectedValue>

    @Olumlu
    Examples: 
      | process_description    | account_number | wallet_number           | url             | method | verifyKey | expectedValue |
      | normal cüzdan sorgusu  |     2638386867 |              1087615544 | /v1/wallet/info | POST   | status    |             0 |
      | loyalty cüzdan sorgusu |     2722802978 | LYT_8167239205922663437 | /v1/wallet/info | POST   | status    |             0 |
      # vs...
    @Olumsuz
    Examples: 
      | process_description           | account_number | wallet_number | url             | method | verifyKey | expectedValue |
      | account_num. boş gönderilirse |                |    1087615544 | /v1/wallet/info | POST   | status    |             0 |
      | wallet_num. boş gönderilirse  |     2722802978 |               | /v1/wallet/info | POST   | status    |             0 |
			# vs...
      
package customKeyword

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import com.kms.katalon.core.testobject.ResponseObject
import internal.GlobalVariable
import customKeyword.Enums
import customKeyword.Verify
import customKeyword.CustomMethods

import com.kms.katalon.core.util.KeywordUtil;
import com.google.gson.Gson;


public class Account {
	public String AccountNumber;
	public String DefaultWalletNumber;

	public Enums.AccountType AccountType = null;
	public KycInfo kycInfo;

	public List<Wallet> Wallets;
	public Wallet BeforeWallet;
	public Wallet AfterWallet;

	public Account() {
		kycInfo = new KycInfo();
		Wallets = new ArrayList<Wallet>();  // tüm cüzdan listesi

		BeforeWallet = new Wallet(); // default cüzdan
		AfterWallet = new Wallet();  // default cüzdan
	}


	public void getAccountByEmail(String email, Enums.AccountType accountType=null, String postfix="mail.com",
			String personalPattern = "_p_", String businessPattern = "_b_") {

		String[] x = email.split("@");

		if(accountType == null) {
			//accountType = email.toLowerCase().contains("_p_") ? Enums.AccountType.personal : email.toLowerCase().contains("_b_") ? Enums.AccountType.business : null
			accountType = DefineAccountType(email, personalPattern, businessPattern)
			println("Account Type : " + accountType)
		}

		if(x.length == 1)
			email +="@"+postfix;


		if(accountType == Enums.AccountType.personal) {

			ResponseObject response = CustomMethods.sendRequest("/v1/Account/PersonalListByFilter", """{
				'id' :'0',
				'tenant_id' :'0',
				'account_id' :'0',
				'email' : '${email}',
				'page_size' :'50',
				'page_index' :'1',
				'order_column' :'Id',
				'order_by' :'asc'
			}""")

			/*******************************************************/
			Verify.VerifyResponseStatusCode('0', response)
			/*******************************************************/

			WS.verifyGreaterThan(WS.getElementPropertyValue(response, "payload.results").size(),0)

			AccountNumber =	WS.getElementPropertyValue(response, "payload.results[0].account_number")
			AccountType = accountType
			kycInfo.FirstName = WS.getElementPropertyValue(response, "payload.results[0].user_kyc_info.first_name")
			kycInfo.LastName = WS.getElementPropertyValue(response, "payload.results[0].user_kyc_info.last_name")
			kycInfo.NationalID = WS.getElementPropertyValue(response, "payload.results[0].user_kyc_info.national_id")
			kycInfo.kycLevel = Enums.KycLevel.valueOf(WS.getElementPropertyValue(response, "payload.results[0].kyc_level"))


			ResponseObject WalletListResponse = getWalletList(AccountNumber)

			ArrayList walletList = WS.getElementPropertyValue(WalletListResponse, "payload.results")
			//println("walletList type : "+walletList.getClass() + "\nsize : "+ walletList.size())

			getDefaultWalletNumber()
			setWallets( walletList, accountType)
			AddAccountList(AccountNumber, this)

			println("wallets : "+ new Gson().toJson(Wallets) )
		}else if(accountType == Enums.AccountType.business) {

			ResponseObject response = CustomMethods.sendRequest("/v1/Account/BusinessListByFilter", """{
				'tanant_id': 0,
				'email' :  "${email}",
				'page_size' : 5,
				'page_index' : 1,
				"order_column": "Id",
				"order_by": "asc"
			}""")

			/*******************************************************/
			Verify.VerifyResponseStatusCode("0",response)
			/*******************************************************/

			WS.verifyGreaterThan(WS.getElementPropertyValue(response, "payload.results").size(),0)

			AccountNumber =	WS.getElementPropertyValue(response, "payload.results[0].account_number")
			AccountType = accountType

			ResponseObject WalletListResponse = getWalletList(AccountNumber)
			ArrayList walletList = WS.getElementPropertyValue(WalletListResponse, "payload.results")

			kycInfo.FirstName = walletList[0]["user_kyc_info"]["first_name"]
			kycInfo.LastName =  walletList[0]["user_kyc_info"]["last_name"]
			kycInfo.NationalID = walletList[0]["user_kyc_info"]["national_id"]
			kycInfo.kycLevel = Enums.KycLevel.valueOf( walletList[0]["kyc_level_status"])

			// println("KycInfo : "+ new Gson().toJson(KycInfo) )

			getDefaultWalletNumber()
			setWallets( walletList, accountType)
			AddAccountList(AccountNumber, this)
			println("wallets : "+ new Gson().toJson(Wallets) )
		}
	}



	/**
	 accountNumber degeri ile igili hesap bilgileri ve cüzdan listesi bulunur.GlobalVariable.accountList güncellenir.
	 */
	public void getAccountByAccountNumber( String accountNumber, Enums.AccountType accountType = Enums.AccountType.personal ) {
		ResponseObject WalletListResponse = getWalletList(accountNumber ); // default accountType : personal

		/*  hesap bulunamadı mesajı ekrana yazılmalı  */
		if(WS.getElementPropertyValue(WalletListResponse, "payload.results").size() == 0 ) {
			KeywordUtil.markWarning(accountNumber + " hesap numarası ile kayıtlı kullanıcı bulanamadı")
			KeywordUtil.logInfo(accountNumber + " hesap numarası ile kayıtlı kullanıcı bulanamadı")
			KeywordUtil.markFailedAndStop(accountNumber + " hesap numarası ile kayıtlı kullanıcı bulanamadı")
		}

		/***********************************************/
		//	WS.verifyGreaterThan(WS.getElementPropertyValue(WalletListResponse, "payload.results").size(),0)

		ArrayList walletList = WS.getElementPropertyValue(WalletListResponse, "payload.results")

		AccountNumber = accountNumber
		AccountType = Enums.AccountType.valueOf(walletList[0]["account_type"].toLowerCase())
		kycInfo.FirstName = walletList[0]["user_kyc_info.first_name"]
		kycInfo.LastName =  walletList[0]["user_kyc_info.last_name"]
		kycInfo.NationalID = walletList[0]["user_kyc_info.national_id"]
		kycInfo.kycLevel = Enums.KycLevel.valueOf( walletList[0]["kyc_level_status"])

		getDefaultWalletNumber()
		setWallets( walletList, accountType)
		AddAccountList(AccountNumber, this)
	}


	/*************************************************************************************/




	public static ResponseObject getWalletList(String accountNumber) {

		ResponseObject response = CustomMethods.sendRequest("/v1/Wallet/ListByFilter", """{ 
			"account_number": "${accountNumber}",
			"page_size": 10,
			"page_index": 1,
			"order_column": "Id",
			"order_by": "asc"
		}""")

		GlobalVariable.responseObject = response
		return response
	}





	public void setWallets(ArrayList walletList, Enums.AccountType accountType = Enums.AccountType.personal) {

		// hem bireysel hem de işletme hesabına ait cüzdan detayı dönüp dönmediği kontrolü

		Dictionary accountTypes = new Hashtable();
		for(int i = 0 ; i<walletList.size() ; i++){
			accountTypes.put(walletList[i]["account_type"].toString().toLowerCase(), i)  // value degeri account type indexi
		}

		// önceki değerlerin üzerine eklenmemesi için liste temizlenir yeni değerler eklenir.
		Wallets.removeAll(Wallets)

		if(accountTypes.keys().size() ==2 ) {
			AccountType = accountType

			//println("hesap türü index : "+accountTypes[String.valueOf(accountType.toString().toLowerCase())])

			kycInfo.FirstName = walletList[accountTypes[String.valueOf(accountType.toString().toLowerCase())]]["user_kyc_info.first_name"]
			kycInfo.LastName =  walletList[accountTypes[String.valueOf(accountType.toString().toLowerCase())]]["user_kyc_info.last_name"]
			kycInfo.NationalID = walletList[accountTypes[String.valueOf(accountType.toString().toLowerCase())]]["user_kyc_info.national_id"]
			kycInfo.kycLevel = Enums.KycLevel.valueOf( walletList[accountTypes[String.valueOf(accountType.toString().toLowerCase())]]["kyc_level_status"])


			// Wallet listesine parametrede belirtilen hesap türüne ait olan cüzdan bilgileri eklenir

			for(int i=0 ; i<walletList.size() ; i++) {

				Wallet w = new Wallet();

				if(walletList[i]["account_type"].toString().toLowerCase() == accountType.toString().toLowerCase()) {
					//	println("Wallet list account type = "+ accountType)

					w.Name = walletList[i]["name"];
					w.Number = walletList[i]["number"];
					w.AccessLevelStatus =  walletList[i]["access_level_status"];

					Balance b = new Balance();
					b.available_balance =  (walletList[i]["payment_balance"]["available"])
					b.unavailable_balance =  (walletList[i]["payment_balance"]["unavailable"])
					w.PaymentBalance = b;

					b=new Balance();
					b.available_balance = (walletList[i]["cash_balance"]["available"])
					b.unavailable_balance =  (walletList[i]["cash_balance"]["unavailable"])
					w.CashBalance = b;

					TransactionLimits l= new TransactionLimits()
					l.max_balance = (walletList[i]["transaction_limits"]["max_balance"])
					l.topup_credit_limit = (walletList[i]["transaction_limits"]["topup_credit_limit"])
					l.topup_cash_limit = (walletList[i]["transaction_limits"]["topup_cash_limit"])
					l.withdrawal_limit = (walletList[i]["transaction_limits"]["withdrawal_limit"])
					l.payment_limit = (walletList[i]["transaction_limits"]["payment_limit"])
					l.wallet_to_wallet_limit = (walletList[i]["transaction_limits"]["wallet_to_wallet_limit"])
					w.Limits = l;

					w.TotalBalance = (walletList[i]["total_balance"])

					Wallets.add(w);
				}
			}

		}else    {
			// tek bir hesap türüne ait cüzdanlar listelenmektedir

			for(int i=0 ; i<walletList.size() ; i++) {

				Wallet w = new Wallet();

				w.Name = walletList[i]["name"];
				w.Number = walletList[i]["number"];
				w.AccessLevelStatus =  walletList[i]["access_level_status"];

				Balance b = new Balance();
				b.available_balance =  (walletList[i]["payment_balance"]["available"])
				b.unavailable_balance =  (walletList[i]["payment_balance"]["unavailable"])
				w.PaymentBalance = b;

				b=new Balance();
				b.available_balance = (walletList[i]["cash_balance"]["available"])
				b.unavailable_balance =  (walletList[i]["cash_balance"]["unavailable"])
				w.CashBalance = b;

				TransactionLimits l= new TransactionLimits()
				l.max_balance = (walletList[i]["transaction_limits"]["max_balance"])
				l.topup_credit_limit = (walletList[i]["transaction_limits"]["topup_credit_limit"])
				l.topup_cash_limit = (walletList[i]["transaction_limits"]["topup_cash_limit"])
				l.withdrawal_limit = (walletList[i]["transaction_limits"]["withdrawal_limit"])
				l.payment_limit = (walletList[i]["transaction_limits"]["payment_limit"])
				l.wallet_to_wallet_limit = (walletList[i]["transaction_limits"]["wallet_to_wallet_limit"])
				w.Limits = l;

				w.TotalBalance = (walletList[i]["total_balance"])

				Wallets.add(w);
			}
		}

		println("setWallets sonrası\nwallets : "+ new Gson().toJson(Wallets) )
	}



	public static Enums.AccountType DefineAccountType(String content, String personalPattern = "_p_",String businessPattern="_b_") {

		println("---------------  DefineAccountType -------------")
		println("content: "+content + " personalPattern: "+personalPattern +" businessPattern: "+businessPattern)

		if(personalPattern!=null && content.toLowerCase().contains(personalPattern.toLowerCase()))
			return Enums.AccountType.personal
		else if(businessPattern!=null && content.toLowerCase().contains(businessPattern.toLowerCase()))
			return Enums.AccountType.business

		throw new Exception("hesap türü belirlenemedi")
	}



	// accountNumber degeri alındığına göre wallet list servisi çağırılıp kyc bilgisi alınabilir
	// hesap türüne göre kontrol yapısına gerek kalmaz
	public static Enums.KycLevel GetKYCStatus(String accountNumber, Enums.AccountType accountType  ) {

		Enums.KycLevel kyc_level_status = Enums.KycLevel.none
		if(!CustomMethods.isNullOrEmpty(accountNumber)) {

			if(accountType.equals(Enums.AccountType.personal)) {
				ResponseObject GetPersonalAccountResponse = CustomMethods.sendRequest("/v1/Account/GetPersonalAccount","""{
					"account_number": "${accountNumber}"
				}""")

				try {
					kyc_level_status = Enums.KycLevel.valueOf( WS.getElementPropertyValue(GetPersonalAccountResponse, "payload.kyc_level") )
				}catch(Exception ex) {
					kyc_level_status = Enums.KycLevel.none
				}
			}else {
				def GetBusinessAccountResponse = CustomMethods.sendRequest("/v1/Account/GetBusinessAccount","""{
					"account_number": "${accountNumber}"
				}""")

				try {
					kyc_level_status = Enums.KycLevel.valueOf(  WS.getElementPropertyValue(GetBusinessAccountResponse, "payload.kyc_level") )
				}catch(Exception ex) {
					kyc_level_status = Enums.KycLevel.none
				}
			}
		}
		return kyc_level_status
	}


	public VerifyKyc(String level) {
		switch(level){
			case "10" :
				WS.verifyEqual(kycInfo.kycLevel, Enums.KycLevel.Unknown)
				break;
			case "20" :
				WS.verifyEqual(kycInfo.kycLevel, Enums.KycLevel.Unverified)
				break;
			case "30" :
				WS.verifyEqual(kycInfo.kycLevel, Enums.KycLevel.Verified)
				break;
			case "40" :
				WS.verifyEqual(kycInfo.kycLevel, Enums.KycLevel.Contracted)
				break;
			default :
				throw new Exception("Kyc tanımlanamadı (parametre ya da account nesnesi kyc bilgisi hatalı)")
		}

	}


	public static void AddAccountList(String accountNumber, Account account) {

		if(!CustomMethods.isNullOrEmpty(accountNumber) && account != null)
		{

			if(GlobalVariable.accountList == null ) {
				GlobalVariable.accountList = new HashMap<String, Account>();
			}

			((HashMap)GlobalVariable.accountList).put(accountNumber, account)

		}
	}


	/**
	 * accountNumber boş ise Account nesnesinde tutulan accountnumber degeri kullanılır
	 * @param accountNumber
	 * @return default wallet number
	 */
	public String CheckPersonalWallet(String accountNumber=null, String currencyCode='TRY') {
		accountNumber = accountNumber == null ? AccountNumber : accountNumber

		if(accountNumber == AccountNumber && Wallets.size() == 1) {
			return Wallets[0].Number
		}

		ResponseObject response = CustomMethods.sendRequest("/v1/Account/CheckPersonalWallet", """{
		 "currency_code": "TRY", 
		 "account_number": "${accountNumber}"
		}""")

		DefaultWalletNumber = WS.getElementPropertyValue(response, "payload.wallet_number")
		return DefaultWalletNumber
	}


	/**
	 * accountNumber boş ise Account nesnesinde tutulan accountnumber degeri kullanılır
	 * @param accountNumber
	 * @return default wallet number
	 */
	public String CheckBusinessWallet(String accountNumber=null, String currencyCode='TRY') {
		accountNumber = accountNumber == null ? AccountNumber : accountNumber

		if(accountNumber == AccountNumber && Wallets.size() == 1) {
			return Wallets[0].Number
		}

		ResponseObject response = CustomMethods.sendRequest("/v1/Account/CheckBusinessWallet", """{
		 "currency_code": "TRY", 
		 "account_number": "${accountNumber}"
		}""")

		DefaultWalletNumber = WS.getElementPropertyValue(response, "payload.wallet_number")
		return DefaultWalletNumber
	}


	public String getDefaultWalletNumber() {
		if(AccountType == Enums.AccountType.personal ) {
			// chackpersonal servisi çağırılmalı dönen responsedaki cüzdan no dönmeli
			return CheckPersonalWallet()
		}else if(AccountType == Enums.AccountType.business) {
			return CheckBusinessWallet()
		}
	}





	public Wallet setBeforeWallet(){

		if(Wallets.size() == 1) { // default cüzdan için checkP/BWallet servisini çağırmaya gerek yok

			BeforeWallet = Wallets[0]
			return BeforeWallet
		}else {
			// cüzdan sayısı 1den fazla ise default cüzdanı bulmak için checkP/BWallet servisi çağırılır
			String defaultWallet = AccountType == Enums.AccountType.personal ? CheckPersonalWallet() : CheckBusinessWallet()
			println("default walllet : "+ defaultWallet)

			/*
			 println("Wallets lenght :"+Wallets.size())
			 for(Wallet w : Wallets ) {
			 println("wallet list -> w.num :"+w.Number + "\nbakiye nb: "+w.CashBalance.unavailable_balance)
			 }	
			 */

			// wallet listesinden default wallet bilgisi beforeWallet'e aktarılır

			for(Wallet w : Wallets ) {
				if(w.Number == defaultWallet) {
					BeforeWallet = w
					return BeforeWallet
				}
			}

		}
	}


	public Wallet setAfterWallet(){
		// wallet listesi çekilmeli
		ResponseObject WalletListResponse = getWalletList(AccountNumber)
		ArrayList walletList = WS.getElementPropertyValue(WalletListResponse, "payload.results")

		//  Account.Wallets prop. yeniden atandı
		setWallets(walletList, AccountType)

		if(walletList.size() == 1) {
			// default cüzdan için checkP/BWallet servisini çağırmaya gerek yok
			AfterWallet = Walles[0]
			return AfterWallet
		}else {
			// cüzdan sayısı 1den fazla ise default cüzdanı bulmak için checkP/BWallet servisi çağırılır
			String defaultWallet = AccountType == Enums.AccountType.personal ? CheckPersonalWallet() : CheckBusinessWallet()
			println("default walllet : "+ defaultWallet)

			println("Wallets lenght :"+Wallets.size())
			for(Wallet w : Wallets ) {
				println("wallet list -> w.num :"+w.Number + "\nbakiye bilgileri \nnb: "+w.CashBalance.unavailable_balance
						+ "\nna"+w.CashBalance.available_balance)
			}

			// wallets listesinden default wallet bulunup after wallet'a aktarılmalı
			for(Wallet w : Wallets ) {
				if(w.Number == defaultWallet) {
					AfterWallet = w
					return AfterWallet
				}
			}

		}
	}







}



public class Wallet {
	String Name;
	String Number;
	String AccessLevelStatus;
	Balance PaymentBalance;
	Balance CashBalance;
	TransactionLimits Limits;
	double TotalBalance;

	public Wallet() {
		PaymentBalance = new Balance();
		CashBalance = new Balance();
		Limits = new TransactionLimits();
	}
}

public class Balance{
	double available_balance = 0;
	double unavailable_balance = 0;
}

public class TransactionLimits{
	double max_balance;
	double topup_credit_limit;
	double topup_cash_limit;
	double withdrawal_limit;
	double payment_limit;
	double wallet_to_wallet_limit;
}

public class KycInfo {
	String FirstName;
	String LastName;
	Enums.KycLevel kycLevel;
	String NationalID;
}

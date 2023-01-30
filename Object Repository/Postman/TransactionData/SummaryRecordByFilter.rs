<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SummaryRecordByFilter</name>
   <tag></tag>
   <elementGuidId>62e67423-e5a5-4d4e-9884-05f3fd427b58</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id}\&quot;,\n  \&quot;ext_transaction_id\&quot;: \&quot;${ext_transaction_id}\&quot;,\n  \&quot;tenant_id\&quot;: \&quot;${tenant_id}\&quot;,\n  \&quot;wallet_id\&quot;: \&quot;${wallet_id}\&quot;,\n  \&quot;process_level_status_id\&quot;: null,\n  \&quot;transaction_type_code\&quot;: \&quot;${transaction_type_code}\&quot;,\n  \&quot;to_wallet_id\&quot;: \&quot;${to_wallet_id}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;, \n  \&quot;from_account_number\&quot;: \&quot;${from_account_number}\&quot;,\n  \&quot;from_account_type_Id\&quot;: \&quot;${from_account_type_Id}\&quot;,\n  \&quot;from_account_Id\&quot;: \&quot;${from_account_Id}\&quot;,\n  \&quot;from_wallet_number\&quot;: \&quot;${from_wallet_number}\&quot;,\n  \&quot;to_account_type_Id\&quot;: \&quot;${to_account_type_Id}\&quot;,\n  \&quot;to_account_Id\&quot;: \&quot;${to_account_Id}\&quot;,\n  \&quot;to_account_number\&quot;: \&quot;${to_account_number}\&quot;,\n  \&quot;to_wallet_number\&quot;: \&quot;${to_wallet_number}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n  \&quot;transaction_direction\&quot;: \&quot;${transaction_direction}\&quot;,\n  \&quot;start_date\&quot;: \&quot;${start_date}\&quot;,\n  \&quot;end_date\&quot;: \&quot;${end_date}\&quot;,\n  \&quot;min_amount\&quot;: \&quot;${min_amount}\&quot;,\n  \&quot;max_amount\&quot;: \&quot;${max_amount}\&quot;,\n  \&quot;media_identifier\&quot;: \&quot;${media_identifier}\&quot;,\n  \&quot;provider_id\&quot;: \&quot;${provider_id}\&quot;,\n  \&quot;page_size\&quot;: \&quot;${page_size}\&quot;,\n  \&quot;page_index\&quot;: \&quot;${page_index}\&quot;,\n  \&quot;order_column\&quot;: \&quot;${order_column}\&quot;,\n  \&quot;order_by\&quot;: \&quot;${order_by}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f6917922-21b7-4a56-83e0-33e0da573a04</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>50f21dd7-b499-4f3a-8754-1f1837241ff1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/TransactionData/SummaryRecordByFilter</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>8394aa49-d68f-4210-be26-62071a00fa48</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>5eb110e6-8382-4916-b469-3fc188ac81ab</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>d150ff1a-ec6b-4ca8-afed-df9287c4cb52</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'60078050055050010852'</defaultValue>
      <description></description>
      <id>72e87e25-5aa8-41b5-a8e5-4d74635097cd</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>3378ca90-80db-4687-a6de-4cc4441cd157</id>
      <masked>false</masked>
      <name>tenant_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>723ef23f-3700-4725-82c3-a1478d3760f7</id>
      <masked>false</masked>
      <name>wallet_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a6ac3a9e-a491-4078-a7ef-8b982ecc69fc</id>
      <masked>false</masked>
      <name>process_level_status_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>157361bc-6a7c-410d-97d6-4f590fe4f280</id>
      <masked>false</masked>
      <name>transaction_type_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>58fe99f9-9135-411a-8ab0-e435155f1dda</id>
      <masked>false</masked>
      <name>to_wallet_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b5905431-160f-47a3-8607-15a24ea8dd6d</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>056b07ce-2dd2-4471-a2c6-4433674d3cce</id>
      <masked>false</masked>
      <name>from_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a01b519b-b7dc-4606-b686-9998f46382cf</id>
      <masked>false</masked>
      <name>from_account_type_Id</name>
   </variables>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>31d08ead-2f68-447b-a2f7-1471dd48c971</id>
      <masked>false</masked>
      <name>from_account_Id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8f0b5a1a-4e00-4e54-9a3c-7d04486e96ac</id>
      <masked>false</masked>
      <name>from_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a3334e6e-775a-470e-8f83-bbcf9f23d509</id>
      <masked>false</masked>
      <name>to_account_type_Id</name>
   </variables>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>12f27daf-50c0-40f6-9790-e9dbc5481815</id>
      <masked>false</masked>
      <name>to_account_Id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>debf1691-9acb-465f-98ed-410c43adaeb6</id>
      <masked>false</masked>
      <name>to_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>591c9221-c99b-48f2-9f49-f9f486fa5e74</id>
      <masked>false</masked>
      <name>to_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e8e97ad4-eef6-4eea-b7ee-e9374e057002</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>62f83af2-2e2a-46b2-ba3a-16efa5e43065</id>
      <masked>false</masked>
      <name>transaction_direction</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>193892f0-9ccf-4b2e-83a5-de4439e0c5db</id>
      <masked>false</masked>
      <name>start_date</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>da66cb0b-f983-4f13-bca7-eedda4f48e47</id>
      <masked>false</masked>
      <name>end_date</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ed52fb67-2779-4657-b0a5-98d987c2ae2d</id>
      <masked>false</masked>
      <name>min_amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>771f0c61-5bde-4036-9a0d-751cc6fac5eb</id>
      <masked>false</masked>
      <name>max_amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>01725162-24d4-4226-84bd-b221a49d7364</id>
      <masked>false</masked>
      <name>media_identifier</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e03ce540-13b2-4c26-85dd-dfd355e72a21</id>
      <masked>false</masked>
      <name>provider_id</name>
   </variables>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>f2053f8d-74f3-43b8-a63a-b506e1f1a3a2</id>
      <masked>false</masked>
      <name>page_size</name>
   </variables>
   <variables>
      <defaultValue>'0'</defaultValue>
      <description></description>
      <id>6c931075-2e30-454d-a409-24b83ae3e2b0</id>
      <masked>false</masked>
      <name>page_index</name>
   </variables>
   <variables>
      <defaultValue>'Id'</defaultValue>
      <description></description>
      <id>bedd0312-3c45-4cf8-a9e4-73d98d863039</id>
      <masked>false</masked>
      <name>order_column</name>
   </variables>
   <variables>
      <defaultValue>'asc'</defaultValue>
      <description></description>
      <id>32d688a2-9934-401e-a202-b875e784d5a8</id>
      <masked>false</masked>
      <name>order_by</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.responseObject = response
GlobalVariable.requestObject = request


println(&quot;SummaryRecordByFilter request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)


import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;SummaryRecordByFilter response body\n&quot;+json.toString(4));


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

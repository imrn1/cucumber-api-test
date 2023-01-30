<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CashBack</name>
   <tag></tag>
   <elementGuidId>dce3a2e6-8864-4e30-8c19-f4bc228c759f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;ext_transaction_id\&quot;:\&quot;${ext_transaction_id}\&quot;,\n    \&quot;sender_account_number\&quot;: \&quot;${sender_account_number}\&quot;,\n    \&quot;sender_wallet_number\&quot;: \&quot;${sender_wallet_number}\&quot;,\n\n    \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n    \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n    \&quot;receiver_wallet_number\&quot;: \&quot;${receiver_wallet_number}\&quot;,\n    \&quot;description\&quot;: \&quot;${description}\&quot;,\n\n    \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;,\n    \&quot;source_type\&quot;: \&quot;${source_type}\&quot;,\n    \&quot;channel_type\&quot;: \&quot;${channel_type}\&quot;\n\n }&quot;,
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
      <webElementGuid>079d33ae-a60b-4166-8c07-efd44698246e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b8645f63-e59d-46dc-b419-c2f452c1a689</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/cashBack</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>78ddbf57-95da-46d9-97a1-6d89fd525bed</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>47b36f86-ef9e-4e85-af0d-7df95787e159</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5e1ca16d-f0b7-4250-9018-b67de6de70a8</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cd53f763-a6f9-4c9c-b281-9e479e064483</id>
      <masked>false</masked>
      <name>sender_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ae1bb3f2-3be9-419c-a9c4-3a0c02dbf887</id>
      <masked>false</masked>
      <name>sender_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d946a107-1b49-4eff-a894-84570fe2b22b</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ff58b485-3845-48a8-a251-fd63c2032e54</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3f788db2-864b-4661-87e4-b5bd89ecb653</id>
      <masked>false</masked>
      <name>receiver_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>92f766ee-e1b6-4550-8a7d-7f5c5af1291b</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9dbeae1f-40c8-406c-b55e-9a9f67184663</id>
      <masked>false</masked>
      <name>hash_key</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>dcfab745-6ac1-4756-92a7-5fbcc6862389</id>
      <masked>false</masked>
      <name>source_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>01ea8cca-95cc-445e-b748-84a956c54677</id>
      <masked>false</masked>
      <name>channel_type</name>
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


println(&quot;cashBack request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)


import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;cashBack response body\n&quot;+json.toString(4)); // Print it with specified indentation


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
